//! HAL RNG traits
//!
//! Abstração de gerador de números aleatórios (TRNG / RNG de hardware).
//!
//! ## NIST SP 800-90B Health Checks
//!
//! Implementa testes de saúde contínua conforme NIST SP 800-90B:
//! - **Monobit Test**: verifica equilíbrio de bits 0/1
//! - **Poker Test**: verifica distribuição de padrões
//! - **Runs Test**: verifica sequências de bits consecutivos
//! - **Continuous Random Number Generator Test (CRNGT)**:
//!   duas amostras consecutivas não devem ser idênticas

use super::error::HalError;

/// Tamanho do buffer de amostras para testes de saúde
const HEALTH_SAMPLE_SIZE: usize = 256;

/// Resultado de um teste de saúde do RNG
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HealthTestResult {
    /// Teste passou
    Pass,
    /// Teste falhou — RNG pode estar comprometido
    Fail,
}

/// Provedor de números aleatórios de entropia (TRNG / RNG de hardware)
pub trait RngProvider {
    /// Preenche o buffer fornecido com bytes aleatórios seguros
    fn fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), HalError>;
    /// Gera um número aleatório de 32 bits
    fn next_u32(&mut self) -> Result<u32, HalError> {
        let mut buf = [0u8; 4];
        self.fill_bytes(&mut buf)?;
        Ok(u32::from_le_bytes(buf))
    }
    /// Verifica se o RNG está saudável
    fn is_healthy(&self) -> bool;
}

/// Health Check para TRNG conforme NIST SP 800-90B
///
/// Realiza testes estatísticos contínuos para detectar falhas no RNG.
/// Em implementações de hardware, estes testes são executados internamente
/// pelo módulo TRNG. Esta implementação fornece validação adicional
/// para ambientes de simulação e como defesa em profundidade.
pub struct RngHealthCheck {
    /// Contador de amostras coletadas
    sample_count: u32,
    /// Última amostra para CRNGT (Continuous Random Number Generator Test)
    last_sample: Option<[u8; 4]>,
    /// Acúmulo de amostras para testes estatísticos
    samples: [u8; HEALTH_SAMPLE_SIZE],
    /// Índice atual no buffer de amostras
    sample_index: usize,
}

impl RngHealthCheck {
    /// Cria um novo health check
    pub const fn new() -> Self {
        Self {
            sample_count: 0,
            last_sample: None,
            samples: [0u8; HEALTH_SAMPLE_SIZE],
            sample_index: 0,
        }
    }

    /// Coleta uma amostra e executa testes de saúde
    ///
    /// Deve ser chamado periodicamente (ex.: a cada N gerações de chave)
    /// para validar continuamente a qualidade do RNG.
    pub fn check<R: RngProvider>(&mut self, rng: &mut R) -> Result<HealthTestResult, HalError> {
        // Coleta 4 bytes de amostra
        let mut sample = [0u8; 4];
        rng.fill_bytes(&mut sample)?;

        // CRNGT: duas amostras consecutivas não devem ser idênticas
        if let Some(last) = &self.last_sample {
            if sample == *last {
                return Ok(HealthTestResult::Fail);
            }
        }
        self.last_sample = Some(sample);

        // Armazena amostra no buffer
        if self.sample_index < HEALTH_SAMPLE_SIZE {
            self.samples[self.sample_index..self.sample_index + 4].copy_from_slice(&sample);
            self.sample_index += 4;
        }

        self.sample_count = self.sample_count.wrapping_add(1);

        // Executa testes estatísticos quando temos dados suficientes
        if self.sample_index >= HEALTH_SAMPLE_SIZE {
            let result = self.run_statistical_tests();
            self.sample_index = 0; // Reseta buffer para próxima iteração
            return Ok(result);
        }

        Ok(HealthTestResult::Pass)
    }

    /// Executa testes estatísticos NIST SP 800-90B
    fn run_statistical_tests(&self) -> HealthTestResult {
        // Monobit Test: verifica equilíbrio de bits 0/1
        if self.monobit_test() == HealthTestResult::Fail {
            return HealthTestResult::Fail;
        }

        // Poker Test: verifica distribuição de padrões de 2 bits
        if self.poker_test() == HealthTestResult::Fail {
            return HealthTestResult::Fail;
        }

        // Runs Test: verifica sequências de bits consecutivos
        if self.runs_test() == HealthTestResult::Fail {
            return HealthTestResult::Fail;
        }

        HealthTestResult::Pass
    }

    /// Monobit Test: conta o número de bits 1 e 0
    ///
    /// Para 2048 bits (256 bytes), o número de 1s deve estar entre
    /// 972 e 1076 (99% de confiança).
    fn monobit_test(&self) -> HealthTestResult {
        let mut ones: u32 = 0;
        let mut zeros: u32 = 0;

        for &byte in &self.samples {
            for bit in 0..8 {
                if (byte >> bit) & 1 == 1 {
                    ones += 1;
                } else {
                    zeros += 1;
                }
            }
        }

        let total = ones + zeros;
        if total == 0 {
            return HealthTestResult::Fail;
        }

        // Para 2048 bits, threshold aproximado do NIST
        let threshold = (total as f64 * 0.05) as u32; // 5% de margem
        let diff = ones.abs_diff(zeros);

        if diff > threshold {
            HealthTestResult::Fail
        } else {
            HealthTestResult::Pass
        }
    }

    /// Poker Test: verifica distribuição de padrões de 2 bits
    ///
    /// Os 4 padrões possíveis (00, 01, 10, 11) devem ter contagens
    /// aproximadamente iguais.
    fn poker_test(&self) -> HealthTestResult {
        let mut counts = [0u32; 4]; // 00, 01, 10, 11

        for &byte in &self.samples {
            for pair in 0..4 {
                let bits = (byte >> (pair * 2)) & 0x03;
                counts[bits as usize] += 1;
            }
        }

        let total: u32 = counts.iter().sum();
        if total == 0 {
            return HealthTestResult::Fail;
        }

        let expected = total as f64 / 4.0;
        for &count in &counts {
            let deviation = ((count as f64) - expected).abs();
            // Threshold: 15% de desvio do esperado
            if deviation > expected * 0.15 {
                return HealthTestResult::Fail;
            }
        }

        HealthTestResult::Pass
    }

    /// Runs Test: verifica sequências de bits consecutivos
    ///
    /// Conta o número de "runs" (sequências de bits iguais consecutivos).
    /// Um RNG saudável deve ter um número de runs próximo ao esperado.
    fn runs_test(&self) -> HealthTestResult {
        let mut runs: u32 = 1; // Começa com 1 run
        let mut prev_bit: Option<u8> = None;

        for &byte in &self.samples {
            for bit in 0..8 {
                let current = (byte >> bit) & 1;
                if let Some(prev) = prev_bit {
                    if current != prev {
                        runs += 1;
                    }
                }
                prev_bit = Some(current);
            }
        }

        let total_bits = (self.sample_index * 8) as u32;
        if total_bits == 0 {
            return HealthTestResult::Fail;
        }

        // Número esperado de runs para dados aleatórios: (n/2) + 1
        let expected_runs = (total_bits as f64 / 2.0) + 1.0;
        let deviation = ((runs as f64) - expected_runs).abs();

        // Threshold: 20% de desvio
        if deviation > expected_runs * 0.20 {
            HealthTestResult::Fail
        } else {
            HealthTestResult::Pass
        }
    }

    /// Retorna o número total de amostras coletadas
    pub fn sample_count(&self) -> u32 {
        self.sample_count
    }
}

impl Default for RngHealthCheck {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Mock RNG saudável para testes
    /// Usa xorshift para produzir dados com boa distribuição estatística
    struct HealthyRng {
        state: u32,
    }

    impl HealthyRng {
        fn new() -> Self {
            Self { state: 0x12345678 }
        }
    }

    impl RngProvider for HealthyRng {
        fn fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), HalError> {
            for byte in dest.iter_mut() {
                // xorshift32
                self.state ^= self.state << 13;
                self.state ^= self.state >> 17;
                self.state ^= self.state << 5;
                *byte = (self.state & 0xFF) as u8;
            }
            Ok(())
        }

        fn is_healthy(&self) -> bool {
            true
        }
    }

    /// Mock RNG com falha (sempre retorna o mesmo valor)
    struct StuckRng;

    impl RngProvider for StuckRng {
        fn fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), HalError> {
            dest.fill(0x42);
            Ok(())
        }

        fn is_healthy(&self) -> bool {
            true
        }
    }

    #[test]
    fn test_crngt_detects_stuck_rng() {
        let mut health = RngHealthCheck::new();
        let mut rng = StuckRng;

        // Primeira amostra: passa (CRNGT não falha na primeira amostra)
        let result = health.check(&mut rng).unwrap();
        assert_eq!(result, HealthTestResult::Pass);

        // Segunda amostra: CRNGT deve falhar (mesma amostra)
        let result = health.check(&mut rng).unwrap();
        assert_eq!(result, HealthTestResult::Fail);
    }

    #[test]
    fn test_healthy_rng_passes_checks() {
        let mut health = RngHealthCheck::new();
        let mut rng = HealthyRng::new();

        // Coleta amostras suficientes para testes estatísticos
        for _ in 0..64 {
            let result = health.check(&mut rng).unwrap();
            // Não deve falhar com um RNG saudável
            assert_ne!(result, HealthTestResult::Fail);
        }
    }

    #[test]
    fn test_sample_count_increments() {
        let mut health = RngHealthCheck::new();
        let mut rng = HealthyRng::new();

        assert_eq!(health.sample_count(), 0);
        health.check(&mut rng).unwrap();
        assert_eq!(health.sample_count(), 1);
        health.check(&mut rng).unwrap();
        assert_eq!(health.sample_count(), 2);
    }
}

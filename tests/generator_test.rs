#[cfg(test)]
mod generator_tests {
    use ch24::generator::{
        find_all_solutions, generate_cardset, is_solvable, NumericExpression, NUM_CARDS, TARGET_NUM,
    };
    const NUM_SAMPLES: usize = 100;

    fn generate_cardset_samples() -> Vec<Vec<NumericExpression>> {
        let mut generated_cardsets = Vec::with_capacity(NUM_SAMPLES);

        for _ in 0..NUM_SAMPLES {
            let cardset = generate_cardset();
            generated_cardsets.push(cardset);
        }

        generated_cardsets
    }

    #[test]
    fn test_it_generates_cardsets() {
        let generated_cardsets = generate_cardset_samples();

        assert!(generated_cardsets.len() == NUM_SAMPLES);
        assert!(generated_cardsets.iter().all(|c| c.len() == NUM_CARDS));
    }

    #[test]
    fn test_it_generates_solvable_cardsets() {
        let generated_cardsets = generate_cardset_samples();

        assert!(generated_cardsets.iter().all(|c| is_solvable(c.clone())));
    }

    #[test]
    fn test_it_generates_valid_solutions() {
        let generated_cardsets = generate_cardset_samples();

        assert!(generated_cardsets
            .iter()
            .all(|c| find_all_solutions(c.clone())
                .iter()
                .all(|s| s.eval() == TARGET_NUM.into())));
    }

  
}

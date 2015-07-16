extern crate item02;

use item02::*;

#[test]
fn instantiation() {
    NutritionFacts::new(240, 8);
}

#[test]
fn instantiation_with_builder() {
    assert!(NutritionFactsBuilder::new(240, 8)
                                  .calories(100)
                                  .sodium_mg(35)
                                  .carbohydrate_g(27)
                                  .build().is_err());
}

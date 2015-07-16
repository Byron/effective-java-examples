
#[derive(Clone)]
pub struct NutritionFacts {
    serving_size_ml: u32,
    servings: u32,
    calories: Option<u32>,
    fat_g: Option<u32>,
    sodium_mg: Option<u32>,
    carbohydrate_g: Option<u32>
}

pub enum ConsistencyError {
    SomethingDidntPanOut(String)
}

pub struct NutritionFactsBuilder {
    inner: NutritionFacts
}

macro_rules! setter {
    ( $( $name:ident ), *) => (
        $(
        pub fn $name(mut self, $name: u32) -> NutritionFactsBuilder {
            self.inner.$name = Some($name);
            self
        }
        )*
    )
}

impl NutritionFactsBuilder {
    pub fn new(serving_size_ml: u32, servings: u32) -> NutritionFactsBuilder {
        NutritionFactsBuilder {
            inner: NutritionFacts::new(serving_size_ml, servings)
        }
    }

    setter!(calories, fat_g, sodium_mg, carbohydrate_g);

    pub fn build(&self) -> Result<NutritionFacts, ConsistencyError> {
        match self.inner.check_consistency() {
            Err(err) => Err(err),
            Ok(())   => Ok(self.inner.clone())
        }
    }
}

impl NutritionFacts {
    pub fn new(serving_size_ml: u32, servings: u32) -> NutritionFacts {
        NutritionFacts {
            serving_size_ml: serving_size_ml,
            servings: servings,
            calories: None,
            fat_g: None,
            sodium_mg: None,
            carbohydrate_g: None,
        }
    }

    pub fn check_consistency(&self) -> Result<(), ConsistencyError> {
        Err(ConsistencyError::SomethingDidntPanOut("This is not really implemented".to_owned()))
    }
}



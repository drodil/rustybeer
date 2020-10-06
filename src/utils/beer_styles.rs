#[derive(Debug, Clone, Copy)]
pub struct BeerStyle<'a> {
    pub name: &'a str,
    pub original_gravity_min: f32,
    pub original_gravity_max: f32,
    pub final_gravity_min: f32,
    pub final_gravity_max: f32,
    pub abv_min: f32,
    pub abv_max: f32,
    pub ibu_min: u8,
    pub ibu_max: u8,
    pub color_srm_min: f32,
    pub color_srm_max: f32,
}

impl<'a> BeerStyle<'a> {
    pub const fn new(name: &'a str, original_gravity_min: f32,
        original_gravity_max: f32, final_gravity_min: f32,
        final_gravity_max: f32, abv_min: f32,
        abv_max: f32, ibu_min: u8, ibu_max: u8,
        color_srm_min: f32, color_srm_max: f32) -> Self {
            Self {
                name,
                original_gravity_min,
                original_gravity_max,
                final_gravity_min,
                final_gravity_max,
                abv_min,
                abv_max,
                ibu_min,
                ibu_max,
                color_srm_min,
                color_srm_max,
            }
    }
}

// Standard American Beer
pub const LITE_AMERICAN_LAGER: BeerStyle = BeerStyle::new("Lite American Lager", 1.028, 1.040, 0.998, 1.008, 2.8, 4.2, 8, 12, 2.0, 3.0);
pub const AMERICAN_LAGER: BeerStyle = BeerStyle::new("American Lager", 1.040, 1.050, 1.004, 1.010, 4.2, 5.3, 8, 18, 2.0, 4.0);
pub const CREAM_ALE: BeerStyle = BeerStyle::new("Cream Ale", 1.042, 1.055, 1.006, 1.012, 4.2, 5.6, 15, 20, 2.5, 5.0);
pub const AMERICAN_WHEAT_BEER: BeerStyle = BeerStyle::new("American Wheat Beer", 1.040, 1.055, 1.008, 1.013, 4.0, 5.5, 15, 30, 3.0, 6.0);

// International Lager
pub const INTERNATIONAL_PALE_LAGER: BeerStyle = BeerStyle::new("International Pale Lager", 1.042, 1.050, 1.008, 1.012, 4.6, 6.0, 18, 25, 2.0, 6.0);
pub const INTERNATIONAL_AMBER_LAGER: BeerStyle = BeerStyle::new("International Amber Lager", 1.042, 1.055, 1.008, 1.014, 4.6, 6.0, 8, 25, 7.0, 14.0);
pub const INTERNATIONAL_DARK_LAGER: BeerStyle = BeerStyle::new("International Dark Lager", 1.044, 1.056, 1.008, 1.012, 4.2, 6.0, 8, 20, 14.0, 22.0);

// Czech Lager
pub const CZECH_PALE_LAGER: BeerStyle = BeerStyle::new("Czech Pale Lager", 1.028, 1.044, 1.008, 1.014, 3.0, 4.1, 20, 35, 3.0, 6.0);
pub const CZECH_PREMIUM_PALE_LAGER : BeerStyle = BeerStyle::new("Czech Premium Pale Lager", 1.044, 1.060, 1.013, 1.017, 4.2, 5.8, 30, 45, 3.5, 6.0);
pub const CZECH_AMBER_LAGER: BeerStyle = BeerStyle::new("Czech Amber Lager", 1.044, 1.060, 1.013, 1.017, 4.2, 5.8, 20, 35, 10.0, 16.0);
pub const CZECH_DARK_LAGER: BeerStyle = BeerStyle::new("Czech Dark Lager", 1.044, 1.060, 1.013, 1.017, 4.4, 5.8, 18, 34, 14.0, 35.0);

// Pale Malty European Lager
pub const MUNICH_HELLES: BeerStyle = BeerStyle::new("Munich Helles", 1.044, 1.048, 1.006, 1.012, 4.7, 5.4, 16, 22, 3.0, 5.0);
pub const FESTBIER: BeerStyle = BeerStyle::new("Festbier", 1.054, 1.057, 1.010, 1.012, 5.8, 6.3, 18, 25, 4.0, 7.0);
pub const HELLES_BOCK: BeerStyle = BeerStyle::new("Helles Bock", 1.064, 1.072, 1.011, 1.018, 6.3, 7.4, 23, 35, 6.0, 11.0);

// Pale Bitter European Lager
pub const GERMAN_LEICHTBIER: BeerStyle = BeerStyle::new("German Leichtbier", 1.026, 1.034, 1.006, 1.010, 2.4, 3.6, 15, 28, 2.0, 5.0);
pub const KOLSCH: BeerStyle = BeerStyle::new("Kölsch", 1.044, 1.050, 1.007, 1.011, 4.4, 5.2, 18, 30, 3.5, 5.0);
pub const GERMAN_EXPORTBIER: BeerStyle = BeerStyle::new("German Exportbier", 1.048, 1.056, 1.010, 1.015, 4.8, 6.0, 20, 30, 4.0, 7.0);
pub const GERMAN_PILS: BeerStyle = BeerStyle::new("German Pils", 1.044, 1.050, 1.008, 1.013, 4.4, 5.2, 22, 40, 2.0, 5.0);

// Amber Malty European Lager
pub const MARZEN: BeerStyle = BeerStyle::new("Märzen", 1.054, 1.060, 1.010, 1.014, 5.8, 6.3, 18, 24, 8.0, 17.0);
pub const RAUCHBIER: BeerStyle = BeerStyle::new("Rauchbier", 1.050, 1.057, 1.012, 1.016, 4.8, 6.0, 20, 30, 12.0, 22.0);
pub const DUNKELS_BOCK: BeerStyle = BeerStyle::new("Dunkels Bock", 1.064, 1.072, 1.013, 1.019, 6.3, 7.2, 20, 27, 14.0, 22.0);

// Amber Bitter European Beer
pub const VIENNA_LAGER: BeerStyle = BeerStyle::new("Vienna Lager", 1.048, 1.055, 1.010, 1.014, 4.7, 5.5, 18, 30, 9.0, 15.0);
pub const ALTBIER: BeerStyle = BeerStyle::new("Altbier", 1.044, 1.052, 1.008, 1.014, 4.3, 5.5, 25, 50, 11.0, 17.0);
pub const MUNICH_KELLERBIER: BeerStyle = BeerStyle::new("Munich Kellerbier", 1.045, 1.051, 1.008, 1.012, 4.7, 5.4, 20, 35, 3.0, 7.0);
pub const FRANCONIAN_KELLERBIER: BeerStyle = BeerStyle::new("Franconian Kellerbier", 1.048, 1.054, 1.012, 1.016, 4.8, 5.4, 25, 40, 7.0, 17.0);

// Dark European Lager
pub const MUNICH_DUNKEL: BeerStyle = BeerStyle::new("Munich Dunkel", 1.048, 1.056, 1.010, 1.016, 4.5, 5.6, 18, 28, 14.0, 28.0);
pub const SCHWARZBIER: BeerStyle = BeerStyle::new("Schwarzbier", 1.046, 1.052, 1.010, 1.016, 4.4, 5.4, 20, 30, 17.0, 30.0);

// Strong European Lager
pub const DOPPELBOCK_PALE: BeerStyle = BeerStyle::new("Doppelbock (Pale)", 1.072, 1.112, 1.016, 1.024, 7.0, 10.0, 16, 26, 6.0, 25.0);
pub const DOPPELBOCK_AMBER: BeerStyle = BeerStyle::new("Doppelbock (Amber)", 1.072, 1.112, 1.016, 1.024, 7.0, 10.0, 16, 26, 6.0, 25.0);
pub const EISBOCK: BeerStyle = BeerStyle::new("Eisbock", 1.078, 1.120, 1.020, 1.035, 9.0, 14.0, 25, 35, 18.0, 30.0);
pub const BALTIC_PORTER: BeerStyle = BeerStyle::new("Baltic Porter", 1.060, 1.090, 1.015, 1.022, 6.5, 9.5, 15, 30, 6.0, 25.0);

// German Wheat Beer
pub const WEIZEN: BeerStyle = BeerStyle::new("Weizen/Weissbier", 1.044, 1.052, 1.010, 1.014, 4.3, 5.6, 8, 15, 2.0, 6.0);
pub const DUNKELS_WEISSBIER: BeerStyle = BeerStyle::new("Dunkels Weissbier", 1.044, 1.056, 1.010, 1.014, 4.3, 5.6, 10, 18, 14.0, 23.0);
pub const WEIZENBOCK: BeerStyle = BeerStyle::new("Weizenbock", 1.064, 1.090, 1.015, 1.022, 6.5, 9.0, 15, 30, 6.0, 25.0);

// English Pale Ale
pub const ORDINARY_BITTER: BeerStyle = BeerStyle::new("Ordinary Bitter", 1.030, 1.039, 1.007, 1.011, 3.2, 3.8, 25, 35, 8.0, 14.0);
pub const BEST_BITTER: BeerStyle = BeerStyle::new("Best Bitter", 1.040, 1.048, 1.008, 1.012, 3.8, 4.6, 25, 40, 8.0, 16.0);
pub const STRONG_BITTER: BeerStyle = BeerStyle::new("Strong Bitter", 1.048, 1.060, 1.010, 1.016, 4.6, 6.2, 30, 50, 8.0, 18.0);

// Pale English Beer
pub const ENGLISH_GOLDEN_ALE: BeerStyle = BeerStyle::new("English Golden Ale", 1.038, 1.053, 1.006, 1.012, 3.8, 5.0, 20, 45, 2.0, 6.0);
pub const AUSTRALIAN_SPARKLING_ALE: BeerStyle = BeerStyle::new("Australian Sparkling Ale", 1.038, 1.050, 1.004, 1.006, 4.5, 6.0, 20, 35, 4.0, 7.0);
pub const ENGLISH_IPA: BeerStyle = BeerStyle::new("English IPA", 1.050, 1.075, 1.010, 1.018, 5.0, 7.5, 40, 60, 6.0, 14.0);

// English Brown Beer
pub const DARK_MILD: BeerStyle = BeerStyle::new("Dark Mild", 1.030, 1.038, 1.008, 1.013, 3.0, 3.8, 10, 25, 12.0, 25.0);
pub const ENGLISH_BROWN_ALE: BeerStyle = BeerStyle::new("English Brown Ale", 1.040, 1.052, 1.008, 1.013, 4.2, 5.4, 20, 30, 12.0, 22.0);
pub const ENGLISH_BROWN_PORTER: BeerStyle = BeerStyle::new("English Brown Porter", 1.040, 1.052, 1.008, 1.014, 4.0, 5.4, 18, 35, 20.0, 30.0);

// Scottish Ale
pub const SCOTTISH_LIGHT: BeerStyle = BeerStyle::new("Scottish Light", 1.030, 1.035, 1.010, 1.013, 2.5, 3.2, 10, 20, 17.0, 22.0);
pub const SCOTTISH_HEAVY: BeerStyle = BeerStyle::new("Scottish Heavy", 1.035, 1.040, 1.010, 1.015, 3.2, 3.9, 10, 20, 13.0, 22.0);
pub const SCOTTISH_EXPORT: BeerStyle = BeerStyle::new("Scottish Export", 1.040, 1.060, 1.010, 1.016, 3.9, 6.0, 15, 30, 13.0, 22.0);

// Irish Beer
pub const IRISH_RED_ALE: BeerStyle = BeerStyle::new("Irish Red Ale", 1.036, 1.046, 1.010, 1.014, 3.8, 5.0, 18, 28, 9.0, 14.0);
pub const IRISH_STOUT: BeerStyle = BeerStyle::new("Irish Stout", 1.036, 1.044, 1.007, 1.011, 4.0, 4.5, 25, 45, 25.0, 45.0);
pub const IRISH_EXTRA_STOUT: BeerStyle = BeerStyle::new("Irish Extra Stout", 1.052, 1.062, 1.010, 1.014, 5.5, 6.5, 35, 50, 25.0, 45.0);

// Dark British Beer
pub const SWEET_STOUT: BeerStyle = BeerStyle::new("Sweet Stout", 1.044, 1.060, 1.012, 1.024, 4.0, 6.0, 20, 40, 30.0, 40.0);
pub const OATMEAL_STOUT: BeerStyle = BeerStyle::new("Oatmeal Stout", 1.045, 1.065, 1.010, 1.018, 4.2, 5.9, 25, 40, 22.0, 40.0);
pub const TROPICAL_STOUT: BeerStyle = BeerStyle::new("Tropical Stout", 1.056, 1.075, 1.010, 1.018, 5.5, 8.0, 30, 50, 30.0, 40.0);
pub const FOREIGN_EXTRA_STOUT: BeerStyle = BeerStyle::new("Foreign Extra Stout", 1.056, 1.075, 1.010, 1.018, 6.3, 8.0, 50, 70, 30.0, 40.0);

// Strong British Ale
pub const ENGLISH_STRONG_ALE: BeerStyle = BeerStyle::new("English Strong Ale", 1.055, 1.080, 1.015, 1.022, 5.5, 8.0, 30, 60, 8.0, 22.0);
pub const OLD_ALE: BeerStyle = BeerStyle::new("Old Ale", 1.055, 1.088, 1.015, 1.022, 5.5, 9.0, 30, 60, 10.0, 22.0);
pub const WEE_HEAVY: BeerStyle = BeerStyle::new("Wee Heavy", 1.070, 1.130, 1.018, 1.040, 6.5, 10.0, 17, 35, 14.0, 25.0);
pub const ENGLISH_BARLEYWINE: BeerStyle = BeerStyle::new("English Barleywine", 1.080, 1.120, 1.018, 1.030, 8.0, 12.0, 35, 70, 8.0, 22.0);

// Pale American Beer
pub const BLONDE_ALE: BeerStyle = BeerStyle::new("Blonde Ale", 1.038, 1.054, 1.008, 1.013, 3.8, 5.5, 15, 28, 3.0, 6.0);
pub const AMERICAN_PALE_ALE: BeerStyle = BeerStyle::new("American Pale Ale", 1.045, 1.060, 1.010, 1.015, 4.5, 6.2, 30, 50, 5.0, 10.0);

// Dark American Lager
pub const AMERICAN_AMBER_ALE: BeerStyle = BeerStyle::new("American Amber Ale", 1.045, 1.060, 1.010, 1.015, 4.5, 6.2, 25, 40, 10.0, 17.0);
pub const CALIFORNIA_COMMON: BeerStyle = BeerStyle::new("California Common", 1.048, 1.054, 1.011, 1.014, 4.5, 5.5, 30, 45, 10.0, 14.0);
pub const AMERICAN_BROWN_ALE: BeerStyle = BeerStyle::new("American Brown Ale", 1.045, 1.060, 1.010, 1.016, 4.3, 6.2, 20, 30, 18.0, 35.0);

// American Porter and Stout
pub const AMERICAN_PORTER: BeerStyle = BeerStyle::new("American Porter", 1.050, 1.070, 1.012, 1.018, 4.8, 6.5, 25, 50, 22.0, 40.0);
pub const AMERICAN_STOUT: BeerStyle = BeerStyle::new("American Stout", 1.050, 1.075, 1.010, 1.022, 5.0, 7.0, 35, 75, 30.0, 40.0);
pub const IMPERIAL_STOUT: BeerStyle = BeerStyle::new("Imperial Stout", 1.075, 1.115, 1.018, 1.030, 8.0, 12.0, 50, 90, 30.0, 40.0);

// IPA
pub const AMERICAN_IPA: BeerStyle = BeerStyle::new("American IPA", 1.056, 1.070, 1.010, 1.014, 5.5, 7.5, 40, 70, 6.0, 14.0);
pub const BELGIAN_IPA: BeerStyle = BeerStyle::new("Belgian IPA", 1.058, 1.080, 1.008, 1.016, 6.2, 9.5, 50, 100, 5.0, 15.0);
pub const BLACK_IPA: BeerStyle = BeerStyle::new("Black IPA", 1.050, 1.085, 1.010, 1.018, 5.5, 9.0, 50, 90, 25.0, 40.0);
pub const BROWN_IPA: BeerStyle = BeerStyle::new("Brown IPA", 1.056, 1.070, 1.008, 1.016, 5.5, 7.5, 40, 70, 11.0, 19.0);
pub const RED_IPA: BeerStyle = BeerStyle::new("Red IPA", 1.056, 1.070, 1.008, 1.016, 5.5, 7.5, 40, 70, 11.0, 19.0);
pub const RYE_IPA: BeerStyle = BeerStyle::new("Rye IPA", 1.056, 1.075, 1.008, 1.014, 5.5, 8.0, 50, 75, 6.0, 14.0);
pub const WHITE_IPA: BeerStyle = BeerStyle::new("White IPA", 1.056, 1.065, 1.010, 1.016, 5.5, 7.0, 40, 70, 5.0, 8.0);

// Strong American Ale
pub const DOUBLE_IPA: BeerStyle = BeerStyle::new("Double IPA", 1.065, 1.085, 1.008, 1.018, 7.5, 10.0, 60, 120, 6.0, 14.0);
pub const AMERICAN_STRONG_ALE: BeerStyle = BeerStyle::new("American Strong Ale", 1.062, 1.090, 1.014, 1.024, 6.3, 10.0, 50, 100, 7.0, 19.0);
pub const AMERICAN_BARLEYWINE: BeerStyle = BeerStyle::new("American Barleywine", 1.080, 1.120, 1.016, 1.030, 8.0, 12.0, 50, 100, 10.0, 19.0);
pub const WHEATWINE: BeerStyle = BeerStyle::new("Wheatwine", 1.080, 1.120, 1.016, 1.030, 8.0, 12.0, 30, 60, 8.0, 15.0);

// European Sour Ale
pub const BERLINER_WEISSE: BeerStyle = BeerStyle::new("Berliner Weisse", 1.028, 1.032, 1.003, 1.006, 2.8, 3.8, 3, 8, 2.0, 3.0);
pub const FLANDERS_RED_ALE: BeerStyle = BeerStyle::new("Flanders Red Ale", 1.048, 1.057, 1.002, 1.012, 4.6, 6.5, 10, 25, 10.0, 16.0);
pub const OUD_BRUIN: BeerStyle = BeerStyle::new("Oud Bruin", 1.040, 1.074, 1.008, 1.012, 4.0, 8.0, 20, 25, 15.0, 22.0);
pub const LAMBIC: BeerStyle = BeerStyle::new("Lambic", 1.040, 1.054, 1.001, 1.010, 5.0, 6.5, 0, 10, 3.0, 7.0);
pub const GUEUZE: BeerStyle = BeerStyle::new("Gueuze", 1.040, 1.060, 1.000, 1.006, 5.0, 8.0, 0, 10, 3.0, 7.0);
pub const FRUIT_LAMBIC: BeerStyle = BeerStyle::new("Fruit Lambic", 1.040, 1.060, 1.000, 1.010, 5.0, 7.0, 0, 10, 3.0, 7.0);

// Belgian Ale
pub const WITBIER: BeerStyle = BeerStyle::new("Witbier", 1.044, 1.052, 1.008, 1.012, 4.5, 5.5, 10, 20, 2.0, 4.0);
pub const BELGIAN_PALE_ALE: BeerStyle = BeerStyle::new("Belgian Pale Ale", 1.048, 1.054, 1.010, 1.014, 4.8, 5.5, 20, 30, 8.0, 14.0);
pub const BIERE_DE_GARDE: BeerStyle = BeerStyle::new("Bière de Garde", 1.060, 1.080, 1.008, 1.016, 6.0, 8.5, 18, 28, 6.0, 9.0);

// Strong Belgian Ale
pub const BELGIAN_BLOND_ALE: BeerStyle = BeerStyle::new("Belgian Blond Ale", 1.062, 1.075, 1.008, 1.018, 6.0, 7.5, 15, 30, 4.0, 7.0);
pub const SAISON: BeerStyle = BeerStyle::new("Saison", 1.048, 1.065, 1.002, 1.008, 5.0, 7.0, 20, 35, 5.0, 14.0);
pub const BELGIAN_GOLDEN_STRONG_ALE: BeerStyle = BeerStyle::new("Belgian Golden Strong Ale", 1.070, 1.095, 1.005, 1.016, 7.5, 10.5, 22, 35, 3.0, 6.0);

// Trappist Ale
pub const BELGIAN_SINGLE: BeerStyle = BeerStyle::new("Belgian Single", 1.044, 1.054, 1.004, 1.010, 4.8, 6.0, 25, 45, 3.0, 5.0);
pub const BELGIAN_DUBBEL: BeerStyle = BeerStyle::new("Belgian Dubbel", 1.062, 1.075, 1.008, 1.018, 6.0, 7.6, 15, 25, 10.0, 17.0);
pub const BELGIAN_TRIPEL: BeerStyle = BeerStyle::new("Belgian Tripel", 1.075, 1.085, 1.008, 1.014, 7.5, 9.5, 20, 40, 4.5, 7.0);
pub const BELGIAN_DARK_STRONG_ALE: BeerStyle = BeerStyle::new("Belgian Dark Strong Ale", 1.075, 1.110, 1.010, 1.024, 8.0, 12.0, 20, 35, 12.0, 22.0);

// Historical Beer
pub const GOSE: BeerStyle = BeerStyle::new("Gose", 1.036, 1.056, 1.006, 1.010, 4.2, 4.8, 5, 12, 3.0, 4.0);
pub const KENTUCKY_COMMON: BeerStyle = BeerStyle::new("Kentucky Common", 1.044, 1.055, 1.010, 1.018, 4.0, 5.5, 15, 30, 11.0, 20.0);
pub const LICHTENHAINER: BeerStyle = BeerStyle::new("Lichtenhainer", 1.032, 1.040, 1.004, 1.008, 3.5, 4.7, 5, 12, 3.0, 6.0);
pub const LONDON_BROWN_ALE: BeerStyle = BeerStyle::new("London Brown Ale", 1.033, 1.038, 1.012, 1.015, 2.8, 3.6, 15, 20, 22.0, 35.0);
pub const PIVO_GRODZISKIE: BeerStyle = BeerStyle::new("Pivo Grodziskie", 1.028, 1.032, 1.006, 1.012, 2.5, 3.3, 20, 35, 3.0, 6.0);
pub const PRE_PROHIBITION_LAGER: BeerStyle = BeerStyle::new("Pre-Prohibition Lager", 1.044, 1.060, 1.010, 1.015, 4.5, 6.0, 25, 40, 3.0, 6.0);
pub const PRE_PROHIBITION_PORTER: BeerStyle = BeerStyle::new("Pre-Prohibition Porter", 1.046, 1.060, 1.010, 1.016, 4.5, 6.0, 20, 30, 18.0, 30.0);
pub const ROGGENBIER: BeerStyle = BeerStyle::new("Roggenbier", 1.046, 1.056, 1.010, 1.014, 4.5, 6.0, 10, 20, 14.0, 19.0);
pub const SAHTI: BeerStyle = BeerStyle::new("Sahti", 1.076, 1.120, 1.016, 1.020, 7.0, 11.0, 7, 15, 4.0, 22.0);

pub static BEER_STYLES: &[BeerStyle] = &[
    LITE_AMERICAN_LAGER, AMERICAN_LAGER, CREAM_ALE, AMERICAN_WHEAT_BEER,
    INTERNATIONAL_PALE_LAGER, INTERNATIONAL_AMBER_LAGER, INTERNATIONAL_DARK_LAGER,
    CZECH_PALE_LAGER, CZECH_PREMIUM_PALE_LAGER, CZECH_AMBER_LAGER, CZECH_DARK_LAGER,
    MUNICH_HELLES, FESTBIER, HELLES_BOCK,
    GERMAN_LEICHTBIER, KOLSCH, GERMAN_EXPORTBIER, GERMAN_PILS,
    MARZEN, RAUCHBIER, DUNKELS_BOCK,
    VIENNA_LAGER, ALTBIER, MUNICH_KELLERBIER, FRANCONIAN_KELLERBIER,
    MUNICH_DUNKEL, SCHWARZBIER,
    DOPPELBOCK_PALE, DOPPELBOCK_AMBER, EISBOCK, BALTIC_PORTER,
    WEIZEN, DUNKELS_WEISSBIER, WEIZENBOCK,
    ORDINARY_BITTER, BEST_BITTER, STRONG_BITTER,
    ENGLISH_GOLDEN_ALE, AUSTRALIAN_SPARKLING_ALE, ENGLISH_IPA,
    DARK_MILD, ENGLISH_BROWN_ALE, ENGLISH_BROWN_PORTER,
    SCOTTISH_LIGHT, SCOTTISH_HEAVY, SCOTTISH_EXPORT,
    IRISH_RED_ALE, IRISH_STOUT, IRISH_EXTRA_STOUT,
    SWEET_STOUT, OATMEAL_STOUT, TROPICAL_STOUT, FOREIGN_EXTRA_STOUT,
    ENGLISH_STRONG_ALE, OLD_ALE, WEE_HEAVY, ENGLISH_BARLEYWINE,
    BLONDE_ALE, AMERICAN_PALE_ALE,
    AMERICAN_AMBER_ALE, CALIFORNIA_COMMON, AMERICAN_BROWN_ALE,
    AMERICAN_PORTER, AMERICAN_STOUT, IMPERIAL_STOUT,
    AMERICAN_IPA, BELGIAN_IPA, BLACK_IPA, BROWN_IPA, RED_IPA, RYE_IPA, WHITE_IPA,
    DOUBLE_IPA, AMERICAN_STRONG_ALE, AMERICAN_BARLEYWINE, WHEATWINE,
    BERLINER_WEISSE, FLANDERS_RED_ALE, OUD_BRUIN, LAMBIC, GUEUZE, FRUIT_LAMBIC,
    WITBIER, BELGIAN_PALE_ALE, BIERE_DE_GARDE,
    BELGIAN_BLOND_ALE, SAISON, BELGIAN_GOLDEN_STRONG_ALE,
    BELGIAN_SINGLE, BELGIAN_DUBBEL, BELGIAN_TRIPEL, BELGIAN_DARK_STRONG_ALE,
    GOSE, KENTUCKY_COMMON, LICHTENHAINER, LONDON_BROWN_ALE, PIVO_GRODZISKIE, PRE_PROHIBITION_LAGER, PRE_PROHIBITION_PORTER, ROGGENBIER, SAHTI,
];

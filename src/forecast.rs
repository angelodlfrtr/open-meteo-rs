use super::{client, errors, forecast, location};
use chrono::TimeZone;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use std::fmt::Display;

#[derive(Debug, Clone)]
pub enum Elevation {
    Nan,
    Value(f32),
}

impl Display for Elevation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Nan => write!(f, "nan"),
            Self::Value(v) => write!(f, "{v}"),
        }
    }
}

impl From<Elevation> for String {
    fn from(value: Elevation) -> Self {
        value.to_string()
    }
}

impl TryFrom<&str> for Elevation {
    type Error = errors::ConversionError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value == "nan" {
            return Ok(Self::Nan);
        }

        Err(errors::ConversionError::InvalidElevation {
            elevation: value.to_string(),
        })
    }
}

impl From<f32> for Elevation {
    fn from(value: f32) -> Self {
        Self::Value(value)
    }
}

#[derive(Debug, Clone)]
pub enum TemperatureUnit {
    Celsius,
    Fahrenheit,
}

impl Display for TemperatureUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Celsius => write!(f, "celsius"),
            Self::Fahrenheit => write!(f, "fahrenheit"),
        }
    }
}

impl From<TemperatureUnit> for String {
    fn from(value: TemperatureUnit) -> Self {
        value.to_string()
    }
}

impl TryFrom<&str> for TemperatureUnit {
    type Error = errors::ConversionError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "celsius" => Ok(Self::Celsius),
            "fahrenheit" => Ok(Self::Fahrenheit),
            _ => Err(errors::ConversionError::InvalidTemperatureUnit {
                unit: value.to_string(),
            }),
        }
    }
}

#[derive(Debug, Clone)]
pub enum WindSpeedUnit {
    Kmh,
    Ms,
    Mph,
    Kn,
}

impl Display for WindSpeedUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Kmh => write!(f, "kmh"),
            Self::Ms => write!(f, "ms"),
            Self::Mph => write!(f, "mph"),
            Self::Kn => write!(f, "kn"),
        }
    }
}

impl From<WindSpeedUnit> for String {
    /// Default to kmh
    fn from(value: WindSpeedUnit) -> Self {
        value.to_string()
    }
}

impl TryFrom<&str> for WindSpeedUnit {
    type Error = errors::ConversionError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "kmh" => Ok(Self::Kmh),
            "ms" => Ok(Self::Ms),
            "mph" => Ok(Self::Mph),
            "kn" => Ok(Self::Kn),
            _ => Err(errors::ConversionError::InvalidWindspeedUnit {
                unit: value.to_string(),
            }),
        }
    }
}

#[derive(Debug, Clone)]
pub enum PrecipitationUnit {
    Millimeters,
    Inches,
}

impl Display for PrecipitationUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Millimeters => write!(f, "mm"),
            Self::Inches => write!(f, "inch"),
        }
    }
}

impl From<PrecipitationUnit> for String {
    /// Default to mm
    fn from(value: PrecipitationUnit) -> Self {
        value.to_string()
    }
}

impl TryFrom<&str> for PrecipitationUnit {
    type Error = errors::ConversionError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "inch" => Ok(Self::Inches),
            "mm" => Ok(Self::Millimeters),
            _ => Err(errors::ConversionError::InvalidPrecipitationUnit {
                unit: value.to_string(),
            }),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Model {
    Undefined,
    BestMatch,
    GfsSeamless,
    GfsGlobal,
    GfsHrrr,
    MeteofranceSeamless,
    MeteofranceArpegeSeamless,
    MeteofranceArpegeWorld,
    MeteofranceArpegeEurope,
    MeteofranceAromeSeamless,
    MeteofranceAromeFrance,
    MeteofranceAromeFranceHd,
    JmaSeamless,
    JmaMsm,
    JmsGsm,
    JmaGsm,
    GemSeamless,
    GemGlobal,
    GemRegional,
    GemHrdpsContinental,
    IconSeamless,
    IconGlobal,
    IconEu,
    IconD2,
    EcmwfIfs04,
    MetnoNordic,
    Era5Seamless,
    Era5,
    Cerra,
    Era5Land,
    EcmwfIfs,
    Gwam,
    Ewam,
    GlofasSeamlessV3,
    GlofasForecastV3,
    GlofasConsolidatedV3,
    GlofasSeamlessV4,
    GlofasForecastV4,
    GlofasConsolidatedV4,
    Gfs025,
    Gfs05,
    CMCCCM2VHR4,
    FGOALSF3HHighressst,
    FGOALSF3H,
    HiramSITHR,
    MRIAGCM32S,
    ECEarth3pHR,
    MPIESM12XR,
    NICAM168S,
    CamsEurope,
    CamsGlobal,
    Cfsv2,
    Era5Ocean,
    CmaGrapesGlobal,
    BomAccessGlobal,
    BomAccessGlobalEnsemble,
    ArpaeCosmoSeamless,
    ArpaeCosmo2i,
    ArpaeCosmo2iRuc,
    ArpaeCosmo5m,
    EcmwfIfs025,
    EcmwfAifs025,
    Gfs013,
    GfsGraphcast025,
    EcmwfWam025,
    MeteofranceWave,
    MeteofranceCurrents,
    EcmwfWam025Ensemble,
    NcepGfswave025,
    NcepGefswave025,
    KnmiSeamless,
    KnmiHarmonieAromeEurope,
    KnmiHarmonieAromeNetherlands,
    DmiSeamless,
    DmiHarmonieAromeEurope,
    MetnoSeamless,
    Era5Ensemble,
    EcmwfIfsAnalysis,
    EcmwfIfsLongWindow,
    EcmwfIfsAnalysisLongWindow,
    UkmoGlobalDeterministic10km,
    UkmoUkDeterministic2km,
    UkmoSeamless,
    NcepGfswave016,
    NcepNbmConus,
    UkmoGlobalEnsemble20km,
    EcmwfAifs025Single,
    JmaJaxaHimawari,
    EumetsatSarah3,
    EumetsatLsaSafMsg,
    EumetsatLsaSafIodc,
    SatelliteRadiationSeamless,
    KmaGdps,
    KmaLdps,
    KmaSeamless,
    ItaliaMeteoArpaeIcon2i,
    UkmoUkEnsemble2km,
    MeteofranceAromeFranceHd15min,
    MeteofranceAromeFrance15min,
    MeteoswissIconCh1,
    MeteoswissIconCh2,
    MeteoswissIconCh1Ensemble,
    MeteoswissIconCh2Ensemble,
    MeteoswissIconSeamless,
    NcepNamConus,
    IconD2Ruc,
    EcmwfSeas5,
    EcmwfEc46,
    EcmwfSeasonalSeamless,
    EcmwfIfsSeamless,
    JmaJaxaMtgFci,
    GemHrdpsWest,
    EcmwfWam,
    NcepAigfs025,
    NcepAigefs025,
    NcepHgefs025EnsembleMean,
    EcmwfSeasonalEnsembleMeanSeamless,
    EcmwfSeas5EnsembleMean,
    EcmwfEc46EnsembleMean,
    NcepAigefs025EnsembleMean,
    DwdIconEpsEnsembleMeanSeamless,
    DwdIconEpsEnsembleMean,
    DwdIconEuEpsEnsembleMean,
    DwdIconD2EpsEnsembleMean,
    NcepGefsEnsembleMeanSeamless,
    NcepGefs025EnsembleMean,
    NcepGefs05EnsembleMean,
    EcmwfIfs025EnsembleMean,
    EcmwfAifs025EnsembleMean,
    MeteoswissIconCh1EnsembleMean,
    MeteoswissIconCh2EnsembleMean,
    CmcGemGepsEnsembleMean,
    UkmoGlobalEnsembleMean20km,
    UkmoUkEnsembleMean2km,
}

impl std::fmt::Display for Model {
    #[allow(clippy::too_many_lines)]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Undefined => write!(f, "undefined"),
            Self::BestMatch => write!(f, "best_match"),
            Self::GfsSeamless => write!(f, "gfs_seamless"),
            Self::GfsGlobal => write!(f, "gfs_global"),
            Self::GfsHrrr => write!(f, "gfs_hrrr"),
            Self::MeteofranceSeamless => write!(f, "meteofrance_seamless"),
            Self::MeteofranceArpegeSeamless => write!(f, "meteofrance_arpege_seamless"),
            Self::MeteofranceArpegeWorld => write!(f, "meteofrance_arpege_world"),
            Self::MeteofranceArpegeEurope => write!(f, "meteofrance_arpege_europe"),
            Self::MeteofranceAromeSeamless => write!(f, "meteofrance_arome_seamless"),
            Self::MeteofranceAromeFrance => write!(f, "meteofrance_arome_france"),
            Self::MeteofranceAromeFranceHd => write!(f, "meteofrance_arome_france_hd"),
            Self::JmaSeamless => write!(f, "jma_seamless"),
            Self::JmaMsm => write!(f, "jma_msm"),
            Self::JmsGsm => write!(f, "jms_gsm"),
            Self::JmaGsm => write!(f, "jma_gsm"),
            Self::GemSeamless => write!(f, "gem_seamless"),
            Self::GemGlobal => write!(f, "gem_global"),
            Self::GemRegional => write!(f, "gem_regional"),
            Self::GemHrdpsContinental => write!(f, "gem_hrdps_continental"),
            Self::IconSeamless => write!(f, "icon_seamless"),
            Self::IconGlobal => write!(f, "icon_global"),
            Self::IconEu => write!(f, "icon_eu"),
            Self::IconD2 => write!(f, "icon_d2"),
            Self::EcmwfIfs04 => write!(f, "ecmwf_ifs04"),
            Self::MetnoNordic => write!(f, "metno_nordic"),
            Self::Era5Seamless => write!(f, "era5_seamless"),
            Self::Era5 => write!(f, "era5"),
            Self::Cerra => write!(f, "cerra"),
            Self::Era5Land => write!(f, "era5_land"),
            Self::EcmwfIfs => write!(f, "ecmwf_ifs"),
            Self::Gwam => write!(f, "gwam"),
            Self::Ewam => write!(f, "ewam"),
            Self::GlofasSeamlessV3 => write!(f, "glofas_seamless_v3"),
            Self::GlofasForecastV3 => write!(f, "glofas_forecast_v3"),
            Self::GlofasConsolidatedV3 => write!(f, "glofas_consolidated_v3"),
            Self::GlofasSeamlessV4 => write!(f, "glofas_seamless_v4"),
            Self::GlofasForecastV4 => write!(f, "glofas_forecast_v4"),
            Self::GlofasConsolidatedV4 => write!(f, "glofas_consolidated_v4"),
            Self::Gfs025 => write!(f, "gfs025"),
            Self::Gfs05 => write!(f, "gfs05"),
            Self::CMCCCM2VHR4 => write!(f, "CMCC_CM2_VHR4"),
            Self::FGOALSF3HHighressst => write!(f, "FGOALS_f3_H_highresSST"),
            Self::FGOALSF3H => write!(f, "FGOALS_f3_H"),
            Self::HiramSITHR => write!(f, "HiRAM_SIT_HR"),
            Self::MRIAGCM32S => write!(f, "MRI_AGCM3_2_S"),
            Self::ECEarth3pHR => write!(f, "EC_Earth3P_HR"),
            Self::MPIESM12XR => write!(f, "MPI_ESM1_2_XR"),
            Self::NICAM168S => write!(f, "NICAM16_8S"),
            Self::CamsEurope => write!(f, "cams_europe"),
            Self::CamsGlobal => write!(f, "cams_global"),
            Self::Cfsv2 => write!(f, "cfsv2"),
            Self::Era5Ocean => write!(f, "era5_ocean"),
            Self::CmaGrapesGlobal => write!(f, "cma_grapes_global"),
            Self::BomAccessGlobal => write!(f, "bom_access_global"),
            Self::BomAccessGlobalEnsemble => write!(f, "bom_access_global_ensemble"),
            Self::ArpaeCosmoSeamless => write!(f, "arpae_cosmo_seamless"),
            Self::ArpaeCosmo2i => write!(f, "arpae_cosmo_2i"),
            Self::ArpaeCosmo2iRuc => write!(f, "arpae_cosmo_2i_ruc"),
            Self::ArpaeCosmo5m => write!(f, "arpae_cosmo_5m"),
            Self::EcmwfIfs025 => write!(f, "ecmwf_ifs025"),
            Self::EcmwfAifs025 => write!(f, "ecmwf_aifs025"),
            Self::Gfs013 => write!(f, "gfs013"),
            Self::GfsGraphcast025 => write!(f, "gfs_graphcast025"),
            Self::EcmwfWam025 => write!(f, "ecmwf_wam025"),
            Self::MeteofranceWave => write!(f, "meteofrance_wave"),
            Self::MeteofranceCurrents => write!(f, "meteofrance_currents"),
            Self::EcmwfWam025Ensemble => write!(f, "ecmwf_wam025_ensemble"),
            Self::NcepGfswave025 => write!(f, "ncep_gfswave025"),
            Self::NcepGefswave025 => write!(f, "ncep_gefswave025"),
            Self::KnmiSeamless => write!(f, "knmi_seamless"),
            Self::KnmiHarmonieAromeEurope => write!(f, "knmi_harmonie_arome_europe"),
            Self::KnmiHarmonieAromeNetherlands => write!(f, "knmi_harmonie_arome_netherlands"),
            Self::DmiSeamless => write!(f, "dmi_seamless"),
            Self::DmiHarmonieAromeEurope => write!(f, "dmi_harmonie_arome_europe"),
            Self::MetnoSeamless => write!(f, "metno_seamless"),
            Self::Era5Ensemble => write!(f, "era5_ensemble"),
            Self::EcmwfIfsAnalysis => write!(f, "ecmwf_ifs_analysis"),
            Self::EcmwfIfsLongWindow => write!(f, "ecmwf_ifs_long_window"),
            Self::EcmwfIfsAnalysisLongWindow => write!(f, "ecmwf_ifs_analysis_long_window"),
            Self::UkmoGlobalDeterministic10km => write!(f, "ukmo_global_deterministic_10km"),
            Self::UkmoUkDeterministic2km => write!(f, "ukmo_uk_deterministic_2km"),
            Self::UkmoSeamless => write!(f, "ukmo_seamless"),
            Self::NcepGfswave016 => write!(f, "ncep_gfswave016"),
            Self::NcepNbmConus => write!(f, "ncep_nbm_conus"),
            Self::UkmoGlobalEnsemble20km => write!(f, "ukmo_global_ensemble_20km"),
            Self::EcmwfAifs025Single => write!(f, "ecmwf_aifs025_single"),
            Self::JmaJaxaHimawari => write!(f, "jma_jaxa_himawari"),
            Self::EumetsatSarah3 => write!(f, "eumetsat_sarah3"),
            Self::EumetsatLsaSafMsg => write!(f, "eumetsat_lsa_saf_msg"),
            Self::EumetsatLsaSafIodc => write!(f, "eumetsat_lsa_saf_iodc"),
            Self::SatelliteRadiationSeamless => write!(f, "satellite_radiation_seamless"),
            Self::KmaGdps => write!(f, "kma_gdps"),
            Self::KmaLdps => write!(f, "kma_ldps"),
            Self::KmaSeamless => write!(f, "kma_seamless"),
            Self::ItaliaMeteoArpaeIcon2i => write!(f, "italia_meteo_arpae_icon_2i"),
            Self::UkmoUkEnsemble2km => write!(f, "ukmo_uk_ensemble_2km"),
            Self::MeteofranceAromeFranceHd15min => write!(f, "meteofrance_arome_france_hd_15min"),
            Self::MeteofranceAromeFrance15min => write!(f, "meteofrance_arome_france_15min"),
            Self::MeteoswissIconCh1 => write!(f, "meteoswiss_icon_ch1"),
            Self::MeteoswissIconCh2 => write!(f, "meteoswiss_icon_ch2"),
            Self::MeteoswissIconCh1Ensemble => write!(f, "meteoswiss_icon_ch1_ensemble"),
            Self::MeteoswissIconCh2Ensemble => write!(f, "meteoswiss_icon_ch2_ensemble"),
            Self::MeteoswissIconSeamless => write!(f, "meteoswiss_icon_seamless"),
            Self::NcepNamConus => write!(f, "ncep_nam_conus"),
            Self::IconD2Ruc => write!(f, "icon_d2_ruc"),
            Self::EcmwfSeas5 => write!(f, "ecmwf_seas5"),
            Self::EcmwfEc46 => write!(f, "ecmwf_ec46"),
            Self::EcmwfSeasonalSeamless => write!(f, "ecmwf_seasonal_seamless"),
            Self::EcmwfIfsSeamless => write!(f, "ecmwf_ifs_seamless"),
            Self::JmaJaxaMtgFci => write!(f, "jma_jaxa_mtg_fci"),
            Self::GemHrdpsWest => write!(f, "gem_hrdps_west"),
            Self::EcmwfWam => write!(f, "ecmwf_wam"),
            Self::NcepAigfs025 => write!(f, "ncep_aigfs025"),
            Self::NcepAigefs025 => write!(f, "ncep_aigefs025"),
            Self::NcepHgefs025EnsembleMean => write!(f, "ncep_hgefs025_ensemble_mean"),
            Self::EcmwfSeasonalEnsembleMeanSeamless => {
                write!(f, "ecmwf_seasonal_ensemble_mean_seamless")
            }
            Self::EcmwfSeas5EnsembleMean => write!(f, "ecmwf_seas5_ensemble_mean"),
            Self::EcmwfEc46EnsembleMean => write!(f, "ecmwf_ec46_ensemble_mean"),
            Self::NcepAigefs025EnsembleMean => write!(f, "ncep_aigefs025_ensemble_mean"),
            Self::DwdIconEpsEnsembleMeanSeamless => {
                write!(f, "dwd_icon_eps_ensemble_mean_seamless")
            }
            Self::DwdIconEpsEnsembleMean => write!(f, "dwd_icon_eps_ensemble_mean"),
            Self::DwdIconEuEpsEnsembleMean => write!(f, "dwd_icon_eu_eps_ensemble_mean"),
            Self::DwdIconD2EpsEnsembleMean => write!(f, "dwd_icon_d2_eps_ensemble_mean"),
            Self::NcepGefsEnsembleMeanSeamless => write!(f, "ncep_gefs_ensemble_mean_seamless"),
            Self::NcepGefs025EnsembleMean => write!(f, "ncep_gefs025_ensemble_mean"),
            Self::NcepGefs05EnsembleMean => write!(f, "ncep_gefs05_ensemble_mean"),
            Self::EcmwfIfs025EnsembleMean => write!(f, "ecmwf_ifs025_ensemble_mean"),
            Self::EcmwfAifs025EnsembleMean => write!(f, "ecmwf_aifs025_ensemble_mean"),
            Self::MeteoswissIconCh1EnsembleMean => write!(f, "meteoswiss_icon_ch1_ensemble_mean"),
            Self::MeteoswissIconCh2EnsembleMean => write!(f, "meteoswiss_icon_ch2_ensemble_mean"),
            Self::CmcGemGepsEnsembleMean => write!(f, "cmc_gem_geps_ensemble_mean"),
            Self::UkmoGlobalEnsembleMean20km => write!(f, "ukmo_global_ensemble_mean_20km"),
            Self::UkmoUkEnsembleMean2km => write!(f, "ukmo_uk_ensemble_mean_2km"),
        }
    }
}

impl TryFrom<&str> for Model {
    type Error = errors::ConversionError;

    #[allow(clippy::too_many_lines)]
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "undefined" => Ok(Self::Undefined),
            "best_match" => Ok(Self::BestMatch),
            "gfs_seamless" => Ok(Self::GfsSeamless),
            "gfs_global" => Ok(Self::GfsGlobal),
            "gfs_hrrr" => Ok(Self::GfsHrrr),
            "meteofrance_seamless" => Ok(Self::MeteofranceSeamless),
            "meteofrance_arpege_seamless" => Ok(Self::MeteofranceArpegeSeamless),
            "meteofrance_arpege_world" => Ok(Self::MeteofranceArpegeWorld),
            "meteofrance_arpege_europe" => Ok(Self::MeteofranceArpegeEurope),
            "meteofrance_arome_seamless" => Ok(Self::MeteofranceAromeSeamless),
            "meteofrance_arome_france" => Ok(Self::MeteofranceAromeFrance),
            "meteofrance_arome_france_hd" => Ok(Self::MeteofranceAromeFranceHd),
            "jma_seamless" => Ok(Self::JmaSeamless),
            "jma_msm" => Ok(Self::JmaMsm),
            "jms_gsm" => Ok(Self::JmsGsm),
            "jma_gsm" => Ok(Self::JmaGsm),
            "gem_seamless" => Ok(Self::GemSeamless),
            "gem_global" => Ok(Self::GemGlobal),
            "gem_regional" => Ok(Self::GemRegional),
            "gem_hrdps_continental" => Ok(Self::GemHrdpsContinental),
            "icon_seamless" => Ok(Self::IconSeamless),
            "icon_global" => Ok(Self::IconGlobal),
            "icon_eu" => Ok(Self::IconEu),
            "icon_d2" => Ok(Self::IconD2),
            "ecmwf_ifs04" => Ok(Self::EcmwfIfs04),
            "metno_nordic" => Ok(Self::MetnoNordic),
            "era5_seamless" => Ok(Self::Era5Seamless),
            "era5" => Ok(Self::Era5),
            "cerra" => Ok(Self::Cerra),
            "era5_land" => Ok(Self::Era5Land),
            "ecmwf_ifs" => Ok(Self::EcmwfIfs),
            "gwam" => Ok(Self::Gwam),
            "ewam" => Ok(Self::Ewam),
            "glofas_seamless_v3" => Ok(Self::GlofasSeamlessV3),
            "glofas_forecast_v3" => Ok(Self::GlofasForecastV3),
            "glofas_consolidated_v3" => Ok(Self::GlofasConsolidatedV3),
            "glofas_seamless_v4" => Ok(Self::GlofasSeamlessV4),
            "glofas_forecast_v4" => Ok(Self::GlofasForecastV4),
            "glofas_consolidated_v4" => Ok(Self::GlofasConsolidatedV4),
            "gfs025" => Ok(Self::Gfs025),
            "gfs05" => Ok(Self::Gfs05),
            "CMCC_CM2_VHR4" => Ok(Self::CMCCCM2VHR4),
            "FGOALS_f3_H_highresSST" => Ok(Self::FGOALSF3HHighressst),
            "FGOALS_f3_H" => Ok(Self::FGOALSF3H),
            "HiRAM_SIT_HR" => Ok(Self::HiramSITHR),
            "MRI_AGCM3_2_S" => Ok(Self::MRIAGCM32S),
            "EC_Earth3P_HR" => Ok(Self::ECEarth3pHR),
            "MPI_ESM1_2_XR" => Ok(Self::MPIESM12XR),
            "NICAM16_8S" => Ok(Self::NICAM168S),
            "cams_europe" => Ok(Self::CamsEurope),
            "cams_global" => Ok(Self::CamsGlobal),
            "cfsv2" => Ok(Self::Cfsv2),
            "era5_ocean" => Ok(Self::Era5Ocean),
            "cma_grapes_global" => Ok(Self::CmaGrapesGlobal),
            "bom_access_global" => Ok(Self::BomAccessGlobal),
            "bom_access_global_ensemble" => Ok(Self::BomAccessGlobalEnsemble),
            "arpae_cosmo_seamless" => Ok(Self::ArpaeCosmoSeamless),
            "arpae_cosmo_2i" => Ok(Self::ArpaeCosmo2i),
            "arpae_cosmo_2i_ruc" => Ok(Self::ArpaeCosmo2iRuc),
            "arpae_cosmo_5m" => Ok(Self::ArpaeCosmo5m),
            "ecmwf_ifs025" => Ok(Self::EcmwfIfs025),
            "ecmwf_aifs025" => Ok(Self::EcmwfAifs025),
            "gfs013" => Ok(Self::Gfs013),
            "gfs_graphcast025" => Ok(Self::GfsGraphcast025),
            "ecmwf_wam025" => Ok(Self::EcmwfWam025),
            "meteofrance_wave" => Ok(Self::MeteofranceWave),
            "meteofrance_currents" => Ok(Self::MeteofranceCurrents),
            "ecmwf_wam025_ensemble" => Ok(Self::EcmwfWam025Ensemble),
            "ncep_gfswave025" => Ok(Self::NcepGfswave025),
            "ncep_gefswave025" => Ok(Self::NcepGefswave025),
            "knmi_seamless" => Ok(Self::KnmiSeamless),
            "knmi_harmonie_arome_europe" => Ok(Self::KnmiHarmonieAromeEurope),
            "knmi_harmonie_arome_netherlands" => Ok(Self::KnmiHarmonieAromeNetherlands),
            "dmi_seamless" => Ok(Self::DmiSeamless),
            "dmi_harmonie_arome_europe" => Ok(Self::DmiHarmonieAromeEurope),
            "metno_seamless" => Ok(Self::MetnoSeamless),
            "era5_ensemble" => Ok(Self::Era5Ensemble),
            "ecmwf_ifs_analysis" => Ok(Self::EcmwfIfsAnalysis),
            "ecmwf_ifs_long_window" => Ok(Self::EcmwfIfsLongWindow),
            "ecmwf_ifs_analysis_long_window" => Ok(Self::EcmwfIfsAnalysisLongWindow),
            "ukmo_global_deterministic_10km" => Ok(Self::UkmoGlobalDeterministic10km),
            "ukmo_uk_deterministic_2km" => Ok(Self::UkmoUkDeterministic2km),
            "ukmo_seamless" => Ok(Self::UkmoSeamless),
            "ncep_gfswave016" => Ok(Self::NcepGfswave016),
            "ncep_nbm_conus" => Ok(Self::NcepNbmConus),
            "ukmo_global_ensemble_20km" => Ok(Self::UkmoGlobalEnsemble20km),
            "ecmwf_aifs025_single" => Ok(Self::EcmwfAifs025Single),
            "jma_jaxa_himawari" => Ok(Self::JmaJaxaHimawari),
            "eumetsat_sarah3" => Ok(Self::EumetsatSarah3),
            "eumetsat_lsa_saf_msg" => Ok(Self::EumetsatLsaSafMsg),
            "eumetsat_lsa_saf_iodc" => Ok(Self::EumetsatLsaSafIodc),
            "satellite_radiation_seamless" => Ok(Self::SatelliteRadiationSeamless),
            "kma_gdps" => Ok(Self::KmaGdps),
            "kma_ldps" => Ok(Self::KmaLdps),
            "kma_seamless" => Ok(Self::KmaSeamless),
            "italia_meteo_arpae_icon_2i" => Ok(Self::ItaliaMeteoArpaeIcon2i),
            "ukmo_uk_ensemble_2km" => Ok(Self::UkmoUkEnsemble2km),
            "meteofrance_arome_france_hd_15min" => Ok(Self::MeteofranceAromeFranceHd15min),
            "meteofrance_arome_france_15min" => Ok(Self::MeteofranceAromeFrance15min),
            "meteoswiss_icon_ch1" => Ok(Self::MeteoswissIconCh1),
            "meteoswiss_icon_ch2" => Ok(Self::MeteoswissIconCh2),
            "meteoswiss_icon_ch1_ensemble" => Ok(Self::MeteoswissIconCh1Ensemble),
            "meteoswiss_icon_ch2_ensemble" => Ok(Self::MeteoswissIconCh2Ensemble),
            "meteoswiss_icon_seamless" => Ok(Self::MeteoswissIconSeamless),
            "ncep_nam_conus" => Ok(Self::NcepNamConus),
            "icon_d2_ruc" => Ok(Self::IconD2Ruc),
            "ecmwf_seas5" => Ok(Self::EcmwfSeas5),
            "ecmwf_ec46" => Ok(Self::EcmwfEc46),
            "ecmwf_seasonal_seamless" => Ok(Self::EcmwfSeasonalSeamless),
            "ecmwf_ifs_seamless" => Ok(Self::EcmwfIfsSeamless),
            "jma_jaxa_mtg_fci" => Ok(Self::JmaJaxaMtgFci),
            "gem_hrdps_west" => Ok(Self::GemHrdpsWest),
            "ecmwf_wam" => Ok(Self::EcmwfWam),
            "ncep_aigfs025" => Ok(Self::NcepAigfs025),
            "ncep_aigefs025" => Ok(Self::NcepAigefs025),
            "ncep_hgefs025_ensemble_mean" => Ok(Self::NcepHgefs025EnsembleMean),
            "ecmwf_seasonal_ensemble_mean_seamless" => Ok(Self::EcmwfSeasonalEnsembleMeanSeamless),
            "ecmwf_seas5_ensemble_mean" => Ok(Self::EcmwfSeas5EnsembleMean),
            "ecmwf_ec46_ensemble_mean" => Ok(Self::EcmwfEc46EnsembleMean),
            "ncep_aigefs025_ensemble_mean" => Ok(Self::NcepAigefs025EnsembleMean),
            "dwd_icon_eps_ensemble_mean_seamless" => Ok(Self::DwdIconEpsEnsembleMeanSeamless),
            "dwd_icon_eps_ensemble_mean" => Ok(Self::DwdIconEpsEnsembleMean),
            "dwd_icon_eu_eps_ensemble_mean" => Ok(Self::DwdIconEuEpsEnsembleMean),
            "dwd_icon_d2_eps_ensemble_mean" => Ok(Self::DwdIconD2EpsEnsembleMean),
            "ncep_gefs_ensemble_mean_seamless" => Ok(Self::NcepGefsEnsembleMeanSeamless),
            "ncep_gefs025_ensemble_mean" => Ok(Self::NcepGefs025EnsembleMean),
            "ncep_gefs05_ensemble_mean" => Ok(Self::NcepGefs05EnsembleMean),
            "ecmwf_ifs025_ensemble_mean" => Ok(Self::EcmwfIfs025EnsembleMean),
            "ecmwf_aifs025_ensemble_mean" => Ok(Self::EcmwfAifs025EnsembleMean),
            "meteoswiss_icon_ch1_ensemble_mean" => Ok(Self::MeteoswissIconCh1EnsembleMean),
            "meteoswiss_icon_ch2_ensemble_mean" => Ok(Self::MeteoswissIconCh2EnsembleMean),
            "cmc_gem_geps_ensemble_mean" => Ok(Self::CmcGemGepsEnsembleMean),
            "ukmo_global_ensemble_mean_20km" => Ok(Self::UkmoGlobalEnsembleMean20km),
            "ukmo_uk_ensemble_mean_2km" => Ok(Self::UkmoUkEnsembleMean2km),
            _ => Err(errors::ConversionError::InvalidModel {
                model: value.to_string(),
            }),
        }
    }
}

#[derive(Debug, Clone)]
pub enum CellSelection {
    Land,
    Sea,
    Nearest,
}

impl Display for CellSelection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Land => write!(f, "land"),
            Self::Sea => write!(f, "sea"),
            Self::Nearest => write!(f, "nearest"),
        }
    }
}

impl From<CellSelection> for String {
    fn from(value: CellSelection) -> Self {
        value.to_string()
    }
}

impl TryFrom<&str> for CellSelection {
    type Error = errors::ConversionError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "land" => Ok(Self::Land),
            "sea" => Ok(Self::Sea),
            "nearest" => Ok(Self::Nearest),
            _ => Err(crate::ConversionError::InvalidCellSelection {
                selection: value.to_string(),
            }),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Options {
    pub location: location::Location,
    pub elevation: Option<Elevation>,
    /// Attributes to request for `minutely_15` forecast
    pub minutely_15: Vec<String>,
    /// Attributes to request in hourly intervals
    pub hourly: Vec<String>,
    /// Attributes to request in daily intervals
    pub daily: Vec<String>,
    /// Attributes to request for current weather
    pub current: Vec<String>,
    pub temperature_unit: Option<TemperatureUnit>,
    pub wind_speed_unit: Option<WindSpeedUnit>,
    pub precipitation_unit: Option<PrecipitationUnit>,
    pub time_zone: Option<String>,
    pub past_days: Option<u8>,
    pub forecast_days: Option<u8>,
    // max minutely_15 data points is 1536
    pub forecast_minutely_15: Option<u16>,
    pub start_date: Option<chrono::NaiveDate>,
    pub end_date: Option<chrono::NaiveDate>,
    pub models: Option<Vec<Model>>,
    pub cell_selection: Option<CellSelection>,
    pub apikey: Option<String>,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            location: location::Location::default(),
            elevation: None,
            minutely_15: Vec::new(),
            hourly: Vec::new(),
            daily: Vec::new(),
            current: Vec::new(),
            temperature_unit: None,
            wind_speed_unit: None,
            precipitation_unit: None,
            time_zone: Some("UTC".into()),
            past_days: None,
            forecast_days: None,
            forecast_minutely_15: None,
            start_date: None,
            end_date: None,
            models: None,
            cell_selection: None,
            apikey: None,
        }
    }
}

impl Options {
    #[must_use]
    pub fn as_params(self) -> Vec<(String, String)> {
        let mut params = Vec::new();

        params.push(("latitude".into(), self.location.lat.to_string()));
        params.push(("longitude".into(), self.location.lng.to_string()));
        params.push(("timeformat".into(), "unixtime".into()));

        if let Some(v) = self.elevation {
            params.push(("elevation".into(), v.into()));
        }

        if let Some(v) = self.temperature_unit {
            params.push(("temperature_unit".into(), v.into()));
        }

        if let Some(v) = self.wind_speed_unit {
            params.push(("windspeed_unit".into(), v.into()));
        }

        if let Some(v) = self.precipitation_unit {
            params.push(("precipitation_unit".into(), v.into()));
        }

        if let Some(v) = self.time_zone {
            params.push(("timezone".into(), v.clone()));
        }

        if let Some(v) = self.past_days {
            params.push(("past_days".into(), v.to_string()));
        }

        if let Some(v) = self.forecast_minutely_15 {
            params.push(("forecast_minutely_15".into(), v.to_string()));
        }

        if let Some(v) = self.forecast_days {
            params.push(("forecast_days".into(), v.to_string()));
        }

        if let Some(v) = self.start_date {
            params.push(("start_date".into(), v.format("%Y-%m-%d").to_string()));
        }

        if let Some(v) = self.end_date {
            params.push(("end_date".into(), v.format("%Y-%m-%d").to_string()));
        }

        if !self.current.is_empty() {
            params.push(("current".into(), self.current.join(",")));
        }

        if !self.minutely_15.is_empty() {
            params.push(("minutely_15".into(), self.minutely_15.join(",")));
        }

        if !self.hourly.is_empty() {
            params.push(("hourly".into(), self.hourly.join(",")));
        }

        if !self.daily.is_empty() {
            params.push(("daily".into(), self.daily.join(",")));
        }

        if let Some(models) = self.models {
            if !models.is_empty() {
                params.push((
                    "models".into(),
                    models
                        .iter()
                        .map(ToString::to_string)
                        .collect::<Vec<_>>()
                        .join(","),
                ));
            }
        }

        if let Some(v) = self.cell_selection {
            params.push(("cell_selection".into(), v.into()));
        }

        if let Some(apikey) = self.apikey {
            params.push(("apikey".into(), apikey.clone()));
        }

        params
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct ApiForecastResponse {
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub elevation: Option<f32>,
    pub generationtime_ms: Option<f64>,
    pub utc_offset_seconds: Option<i32>,
    pub timezone: Option<String>,
    pub timezone_abbreviation: Option<String>,
    pub current_units: Option<HashMap<String, String>>,
    pub current: Option<HashMap<String, serde_json::Value>>,
    pub minutely_15_units: Option<HashMap<String, String>>,
    pub minutely_15: Option<HashMap<String, serde_json::Value>>,
    pub hourly_units: Option<HashMap<String, String>>,
    pub hourly: Option<HashMap<String, serde_json::Value>>,
    pub daily_units: Option<HashMap<String, String>>,
    pub daily: Option<HashMap<String, serde_json::Value>>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ForecastResultItem {
    pub unit: Option<String>,
    pub value: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ForecastResultHourly {
    pub datetime: chrono::NaiveDateTime,
    pub values: HashMap<String, ForecastResultItem>,
}

pub type CurrentResult = ForecastResultHourly;
pub type ForecastResultMinutely15 = ForecastResultHourly;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ForecastResultDaily {
    pub date: chrono::NaiveDate,
    pub values: HashMap<String, ForecastResultItem>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ForecastResult {
    pub current: Option<CurrentResult>,
    pub minutely_15: Option<Vec<ForecastResultMinutely15>>,
    pub hourly: Option<Vec<ForecastResultHourly>>,
    pub daily: Option<Vec<ForecastResultDaily>>,
}

impl client::Client {
    /// Request forecast data
    ///
    /// ### Errors
    ///
    /// Return an `Err` if api return an error or in case of network error.
    pub async fn forecast(&self, opts: Options) -> Result<ForecastResult, Box<dyn Error>> {
        self.request(opts, &format!("{}forecast", self.forecast_endpoint))
            .await
    }

    /// Request data from the archive (historic weather data)
    ///
    /// ### Errors
    ///
    /// Return an `Err` if api return an error or in case of network error.
    pub async fn archive(&self, opts: Options) -> Result<ForecastResult, Box<dyn Error>> {
        self.request(opts, &format!("{}archive", self.archive_endpoint))
            .await
    }

    #[allow(clippy::too_many_lines)]
    async fn request(
        &self,
        opts: Options,
        api_endpoint: &str,
    ) -> Result<ForecastResult, Box<dyn Error>> {
        let url = reqwest::Url::parse_with_params(api_endpoint, opts.as_params())?;
        let res = self.http_client.get(url).send().await?;

        if res.status().is_success() {
            let api_res = res.json::<ApiForecastResponse>().await?;
            let mut result = ForecastResult::default();

            // Current weather
            if let Some(current) = api_res.current {
                let api_units = api_res.current_units.clone();
                // Iterates on values
                let mut current_result = CurrentResult::default();
                for (k, v) in &current {
                    if k == "time" {
                        current_result.datetime = match v.as_i64() {
                            Some(v) => unix_time_to_naive_datetime(v, 0),
                            None => {
                                return Err("cannot decode properly json input".into());
                            }
                        };
                        continue;
                    }
                    // Try to find the unit
                    let unit = api_units.as_ref().and_then(|units| units.get(k).cloned());
                    let value = v.clone();
                    current_result
                        .values
                        .insert(k.clone(), ForecastResultItem { unit, value });
                }

                // Push current rec
                result.current = Some(current_result);
            }

            // Get utc offset
            let utc_offset_seconds = api_res.utc_offset_seconds.unwrap_or(0);

            // Minutely 15
            if let Some(minutely_15) = api_res.minutely_15 {
                if let Some(minutely_15_date_times) =
                    extract_times(&minutely_15, utc_offset_seconds)?
                {
                    if let Some(minutely_15_units) = api_res.minutely_15_units {
                        let mut minutely_15_result = Vec::new();

                        // Iterate on times
                        for (idx, time) in minutely_15_date_times.iter().enumerate() {
                            let mut minutely_15_rec = ForecastResultMinutely15 {
                                datetime: *time,
                                ..Default::default()
                            };

                            // Iterates on values
                            for (k, v) in &minutely_15 {
                                if k == "time" {
                                    continue;
                                }

                                let mut item = ForecastResultItem::default();
                                let Some(v_arr) = v.as_array() else {
                                    return Err("cannot decode properly json input".into());
                                };

                                let v_val = v_arr[idx].clone();
                                item.value = v_val;

                                // Try to find unit
                                if let Some(unit) = minutely_15_units.get(k) {
                                    item.unit = Some(unit.clone());
                                }

                                // Push to minutely_15 record
                                minutely_15_rec.values.insert(k.clone(), item);
                            }

                            // Push minutely_15 rec
                            minutely_15_result.push(minutely_15_rec);
                        }

                        result.minutely_15 = Some(minutely_15_result);
                    }
                }
            }

            // Hourly
            if let Some(hourly) = api_res.hourly {
                if let Some(hourly_date_times) = extract_times(&hourly, utc_offset_seconds)? {
                    if let Some(hourly_units) = api_res.hourly_units {
                        let mut hourly_result = Vec::new();

                        // Iterate on times
                        for (idx, time) in hourly_date_times.iter().enumerate() {
                            let mut hourly_rec = forecast::ForecastResultHourly {
                                datetime: *time,
                                ..Default::default()
                            };

                            // Iterates on values
                            for (k, v) in &hourly {
                                if k == "time" {
                                    continue;
                                }

                                let mut item = ForecastResultItem::default();
                                let Some(v_arr) = v.as_array() else {
                                    return Err("cannot decode properly json input".into());
                                };

                                let v_val = v_arr[idx].clone();
                                item.value = v_val;

                                // Try to find unit
                                if let Some(unit) = hourly_units.get(k) {
                                    item.unit = Some(unit.clone());
                                }

                                // Push to hourly record
                                hourly_rec.values.insert(k.clone(), item);
                            }

                            // Push hourly rec
                            hourly_result.push(hourly_rec);
                        }

                        result.hourly = Some(hourly_result);
                    }
                }
            }

            // Daily
            if let Some(daily) = api_res.daily {
                if let Some(daily_date_times) = extract_times(&daily, utc_offset_seconds)? {
                    if let Some(daily_units) = api_res.daily_units {
                        let mut daily_result = Vec::new();

                        // Iterate on times
                        for (idx, time) in daily_date_times.iter().enumerate() {
                            let mut daily_rec = forecast::ForecastResultDaily {
                                date: (*time).date(),
                                ..Default::default()
                            };

                            // Iterates on values
                            for (k, v) in &daily {
                                if k == "time" {
                                    continue;
                                }

                                let mut item = ForecastResultItem::default();
                                let Some(v_arr) = v.as_array() else {
                                    return Err("cannot decode properly json input".into());
                                };
                                let v_val = v_arr[idx].clone();
                                item.value = v_val;

                                // Try to find unit
                                if let Some(unit) = daily_units.get(k) {
                                    item.unit = Some(unit.clone());
                                }

                                // Push to daily record
                                daily_rec.values.insert(k.clone(), item);
                            }

                            // Push daily rec
                            daily_result.push(daily_rec);
                        }

                        result.daily = Some(daily_result);
                    }
                }
            }

            return Ok(result);
        }

        Err(Box::new(errors::ClientError::InvalidResponseStatus {
            status_code: res.status().as_u16(),
            text: res.text().await.unwrap_or(String::new()),
        }))
    }
}

#[must_use]
pub fn unix_time_to_naive_datetime(
    unix_time: i64,
    utc_offset_seconds: i32,
) -> chrono::NaiveDateTime {
    chrono::Utc
        .timestamp_millis_opt((unix_time + i64::from(utc_offset_seconds)) * 1000)
        .unwrap()
        .naive_local()
}

/// Extract times from json and return a `Option<Vec<chrono::NaiveDateTime>>`.
///
/// ### Errors
///
/// Return `Err` if json input cannot be decoded.
pub fn extract_times<S: ::std::hash::BuildHasher>(
    input: &HashMap<String, serde_json::Value, S>,
    utc_offset_seconds: i32,
) -> Result<Option<Vec<chrono::NaiveDateTime>>, Box<dyn Error>> {
    if let Some(time) = input.get("time") {
        if let Some(time_values) = time.as_array() {
            let mut hourly_datetimes = Vec::new();

            for v in time_values {
                let Some(unix_tm) = v.as_i64() else {
                    return Err("cannot decode properly json input".into());
                };

                let dd = unix_time_to_naive_datetime(unix_tm, utc_offset_seconds);

                hourly_datetimes.push(dd);
            }

            return Ok(Some(hourly_datetimes));
        }
    }

    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Duration;
    use futures::join;

    #[tokio::test]
    async fn get_forecast_single() {
        let clt = client::Client::new();
        let mut opts = Options {
            location: location::Location {
                lat: 52.52,
                lng: 13.41,
            },
            current: vec!["temperature_2m".into()],
            elevation: Some(8.65.into()),
            ..Default::default()
        };

        opts.elevation = Some("nan".try_into().unwrap());

        opts.minutely_15.push("temperature_2m".into());
        opts.minutely_15.push("windspeed_10m".into());
        opts.hourly.push("temperature_2m".into());
        opts.hourly.push("windspeed_120m".into());
        opts.daily.push("temperature_2m_max".into());
        opts.daily.push("shortwave_radiation_sum".into());
        opts.time_zone = Some(chrono_tz::Tz::Europe__Paris.to_string());

        opts.start_date = Some(chrono::Utc::now().date_naive());
        opts.end_date = Some((chrono::Utc::now() + Duration::days(4)).date_naive());

        let res = clt.forecast(opts).await.unwrap();
        println!("{res:#?}");
    }

    #[tokio::test]
    async fn get_forecast_parallel() {
        let clt = client::Client::new();

        let mut opts = Options {
            location: location::Location {
                lat: 48.864_716,
                lng: 2.349_014,
            },
            ..Default::default()
        };

        opts.hourly.push("temperature_2m".into());

        let opts_two = opts.clone();
        let fut_one = clt.forecast(opts);
        let fut_two = clt.forecast(opts_two);

        let (res_one, res_two) = join!(fut_one, fut_two);

        println!("{:?}", res_one.unwrap());
        println!("{:?}", res_two.unwrap());
    }
}

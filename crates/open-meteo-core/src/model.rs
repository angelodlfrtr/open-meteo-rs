//! Weather models accepted by the `models` parameter of most endpoints.
//!
//! Each variant names one model id of the API; its documentation gives the provider and the id.
//! Which models an endpoint accepts is listed on that endpoint's documentation page.

use alloc::string::{String, ToString};
use core::fmt;

use crate::ConversionError;

/// A weather, climate, wave or satellite model known to the API.
///
/// Converts to and from the API's string id with `Display` and `TryFrom<&str>`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Model {
    /// Not a real model; sends `undefined`. Prefer leaving `models` empty.
    Undefined,
    /// Let the API pick the best model for the location.
    BestMatch,
    /// NOAA GFS: `gfs_seamless`.
    GfsSeamless,
    /// NOAA GFS: `gfs_global`.
    GfsGlobal,
    /// NOAA GFS: `gfs_hrrr`.
    GfsHrrr,
    /// Météo-France: `meteofrance_seamless`.
    MeteofranceSeamless,
    /// Météo-France: `meteofrance_arpege_seamless`.
    MeteofranceArpegeSeamless,
    /// Météo-France: `meteofrance_arpege_world`.
    MeteofranceArpegeWorld,
    /// Météo-France: `meteofrance_arpege_europe`.
    MeteofranceArpegeEurope,
    /// Météo-France: `meteofrance_arome_seamless`.
    MeteofranceAromeSeamless,
    /// Météo-France: `meteofrance_arome_france`.
    MeteofranceAromeFrance,
    /// Météo-France: `meteofrance_arome_france_hd`.
    MeteofranceAromeFranceHd,
    /// JMA (Japan): `jma_seamless`.
    JmaSeamless,
    /// JMA (Japan): `jma_msm`.
    JmaMsm,
    /// JMA (Japan): `jms_gsm`.
    JmsGsm,
    /// JMA (Japan): `jma_gsm`.
    JmaGsm,
    /// Environment Canada GEM: `gem_seamless`.
    GemSeamless,
    /// Environment Canada GEM: `gem_global`.
    GemGlobal,
    /// Environment Canada GEM: `gem_regional`.
    GemRegional,
    /// Environment Canada GEM: `gem_hrdps_continental`.
    GemHrdpsContinental,
    /// DWD: `icon_seamless`.
    IconSeamless,
    /// DWD: `icon_global`.
    IconGlobal,
    /// DWD: `icon_eu`.
    IconEu,
    /// DWD: `icon_d2`.
    IconD2,
    /// ECMWF: `ecmwf_ifs04`.
    EcmwfIfs04,
    /// MET Norway: `metno_nordic`.
    MetnoNordic,
    /// ECMWF ERA5 reanalysis: `era5_seamless`.
    Era5Seamless,
    /// ECMWF ERA5 reanalysis: `era5`.
    Era5,
    /// Copernicus CERRA reanalysis: `cerra`.
    Cerra,
    /// ECMWF ERA5 reanalysis: `era5_land`.
    Era5Land,
    /// ECMWF: `ecmwf_ifs`.
    EcmwfIfs,
    /// DWD wave model: `gwam`.
    Gwam,
    /// DWD wave model: `ewam`.
    Ewam,
    /// Copernicus flood awareness system (river discharge): `glofas_seamless_v3`.
    GlofasSeamlessV3,
    /// Copernicus flood awareness system (river discharge): `glofas_forecast_v3`.
    GlofasForecastV3,
    /// Copernicus flood awareness system (river discharge): `glofas_consolidated_v3`.
    GlofasConsolidatedV3,
    /// Copernicus flood awareness system (river discharge): `glofas_seamless_v4`.
    GlofasSeamlessV4,
    /// Copernicus flood awareness system (river discharge): `glofas_forecast_v4`.
    GlofasForecastV4,
    /// Copernicus flood awareness system (river discharge): `glofas_consolidated_v4`.
    GlofasConsolidatedV4,
    /// NOAA GFS: `gfs025`.
    Gfs025,
    /// NOAA GFS: `gfs05`.
    Gfs05,
    /// CMIP6 high-resolution climate model: `CMCC_CM2_VHR4`.
    CMCCCM2VHR4,
    /// CMIP6 high-resolution climate model: `FGOALS_f3_H_highresSST`.
    FGOALSF3HHighressst,
    /// CMIP6 high-resolution climate model: `FGOALS_f3_H`.
    FGOALSF3H,
    /// CMIP6 high-resolution climate model: `HiRAM_SIT_HR`.
    HiramSITHR,
    /// CMIP6 high-resolution climate model: `MRI_AGCM3_2_S`.
    MRIAGCM32S,
    /// CMIP6 high-resolution climate model: `EC_Earth3P_HR`.
    ECEarth3pHR,
    /// CMIP6 high-resolution climate model: `MPI_ESM1_2_XR`.
    MPIESM12XR,
    /// CMIP6 high-resolution climate model: `NICAM16_8S`.
    NICAM168S,
    /// Copernicus CAMS air quality: `cams_europe`.
    CamsEurope,
    /// Copernicus CAMS air quality: `cams_global`.
    CamsGlobal,
    /// NOAA climate forecast system, version 2 (seasonal): `cfsv2`.
    Cfsv2,
    /// ECMWF ERA5 reanalysis: `era5_ocean`.
    Era5Ocean,
    /// CMA (China): `cma_grapes_global`.
    CmaGrapesGlobal,
    /// BOM (Australia): `bom_access_global`.
    BomAccessGlobal,
    /// BOM (Australia): `bom_access_global_ensemble`.
    BomAccessGlobalEnsemble,
    /// Italian weather services (ARPAE, Italia Meteo): `arpae_cosmo_seamless`.
    ArpaeCosmoSeamless,
    /// Italian weather services (ARPAE, Italia Meteo): `arpae_cosmo_2i`.
    ArpaeCosmo2i,
    /// Italian weather services (ARPAE, Italia Meteo): `arpae_cosmo_2i_ruc`.
    ArpaeCosmo2iRuc,
    /// Italian weather services (ARPAE, Italia Meteo): `arpae_cosmo_5m`.
    ArpaeCosmo5m,
    /// ECMWF: `ecmwf_ifs025`.
    EcmwfIfs025,
    /// ECMWF: `ecmwf_aifs025`.
    EcmwfAifs025,
    /// NOAA GFS: `gfs013`.
    Gfs013,
    /// NOAA GFS: `gfs_graphcast025`.
    GfsGraphcast025,
    /// ECMWF WAM wave model: `ecmwf_wam025`.
    EcmwfWam025,
    /// Météo-France: `meteofrance_wave`.
    MeteofranceWave,
    /// Météo-France: `meteofrance_currents`.
    MeteofranceCurrents,
    /// ECMWF WAM wave model: `ecmwf_wam025_ensemble`.
    EcmwfWam025Ensemble,
    /// NOAA NCEP: `ncep_gfswave025`.
    NcepGfswave025,
    /// NOAA NCEP: `ncep_gefswave025`.
    NcepGefswave025,
    /// KNMI (Netherlands): `knmi_seamless`.
    KnmiSeamless,
    /// KNMI (Netherlands): `knmi_harmonie_arome_europe`.
    KnmiHarmonieAromeEurope,
    /// KNMI (Netherlands): `knmi_harmonie_arome_netherlands`.
    KnmiHarmonieAromeNetherlands,
    /// DMI (Denmark): `dmi_seamless`.
    DmiSeamless,
    /// DMI (Denmark): `dmi_harmonie_arome_europe`.
    DmiHarmonieAromeEurope,
    /// MET Norway: `metno_seamless`.
    MetnoSeamless,
    /// ECMWF ERA5 reanalysis: `era5_ensemble`.
    Era5Ensemble,
    /// ECMWF: `ecmwf_ifs_analysis`.
    EcmwfIfsAnalysis,
    /// ECMWF: `ecmwf_ifs_long_window`.
    EcmwfIfsLongWindow,
    /// ECMWF: `ecmwf_ifs_analysis_long_window`.
    EcmwfIfsAnalysisLongWindow,
    /// UK Met Office: `ukmo_global_deterministic_10km`.
    UkmoGlobalDeterministic10km,
    /// UK Met Office: `ukmo_uk_deterministic_2km`.
    UkmoUkDeterministic2km,
    /// UK Met Office: `ukmo_seamless`.
    UkmoSeamless,
    /// NOAA NCEP: `ncep_gfswave016`.
    NcepGfswave016,
    /// NOAA NCEP: `ncep_nbm_conus`.
    NcepNbmConus,
    /// UK Met Office: `ukmo_global_ensemble_20km`.
    UkmoGlobalEnsemble20km,
    /// ECMWF: `ecmwf_aifs025_single`.
    EcmwfAifs025Single,
    /// JMA/JAXA satellite: `jma_jaxa_himawari`.
    JmaJaxaHimawari,
    /// EUMETSAT satellite: `eumetsat_sarah3`.
    EumetsatSarah3,
    /// EUMETSAT satellite: `eumetsat_lsa_saf_msg`.
    EumetsatLsaSafMsg,
    /// EUMETSAT satellite: `eumetsat_lsa_saf_iodc`.
    EumetsatLsaSafIodc,
    /// Open-Meteo satellite blend: `satellite_radiation_seamless`.
    SatelliteRadiationSeamless,
    /// KMA (Korea): `kma_gdps`.
    KmaGdps,
    /// KMA (Korea): `kma_ldps`.
    KmaLdps,
    /// KMA (Korea): `kma_seamless`.
    KmaSeamless,
    /// Italian weather services (ARPAE, Italia Meteo): `italia_meteo_arpae_icon_2i`.
    ItaliaMeteoArpaeIcon2i,
    /// UK Met Office: `ukmo_uk_ensemble_2km`.
    UkmoUkEnsemble2km,
    /// Météo-France: `meteofrance_arome_france_hd_15min`.
    MeteofranceAromeFranceHd15min,
    /// Météo-France: `meteofrance_arome_france_15min`.
    MeteofranceAromeFrance15min,
    /// Swiss weather service: `meteoswiss_icon_ch1`.
    MeteoswissIconCh1,
    /// Swiss weather service: `meteoswiss_icon_ch2`.
    MeteoswissIconCh2,
    /// Swiss weather service: `meteoswiss_icon_ch1_ensemble`.
    MeteoswissIconCh1Ensemble,
    /// Swiss weather service: `meteoswiss_icon_ch2_ensemble`.
    MeteoswissIconCh2Ensemble,
    /// Swiss weather service: `meteoswiss_icon_seamless`.
    MeteoswissIconSeamless,
    /// NOAA NCEP: `ncep_nam_conus`.
    NcepNamConus,
    /// DWD: `icon_d2_ruc`.
    IconD2Ruc,
    /// ECMWF seasonal: `ecmwf_seas5`.
    EcmwfSeas5,
    /// ECMWF seasonal: `ecmwf_ec46`.
    EcmwfEc46,
    /// ECMWF seasonal: `ecmwf_seasonal_seamless`.
    EcmwfSeasonalSeamless,
    /// ECMWF: `ecmwf_ifs_seamless`.
    EcmwfIfsSeamless,
    /// JMA/JAXA satellite: `jma_jaxa_mtg_fci`.
    JmaJaxaMtgFci,
    /// Environment Canada GEM: `gem_hrdps_west`.
    GemHrdpsWest,
    /// ECMWF WAM wave model: `ecmwf_wam`.
    EcmwfWam,
    /// NOAA NCEP: `ncep_aigfs025`.
    NcepAigfs025,
    /// NOAA NCEP: `ncep_aigefs025`.
    NcepAigefs025,
    /// NOAA NCEP: `ncep_hgefs025_ensemble_mean`.
    NcepHgefs025EnsembleMean,
    /// ECMWF seasonal: `ecmwf_seasonal_ensemble_mean_seamless`.
    EcmwfSeasonalEnsembleMeanSeamless,
    /// ECMWF seasonal: `ecmwf_seas5_ensemble_mean`.
    EcmwfSeas5EnsembleMean,
    /// ECMWF seasonal: `ecmwf_ec46_ensemble_mean`.
    EcmwfEc46EnsembleMean,
    /// NOAA NCEP: `ncep_aigefs025_ensemble_mean`.
    NcepAigefs025EnsembleMean,
    /// DWD: `dwd_icon_eps_ensemble_mean_seamless`.
    DwdIconEpsEnsembleMeanSeamless,
    /// DWD: `dwd_icon_eps_ensemble_mean`.
    DwdIconEpsEnsembleMean,
    /// DWD: `dwd_icon_eu_eps_ensemble_mean`.
    DwdIconEuEpsEnsembleMean,
    /// DWD: `dwd_icon_d2_eps_ensemble_mean`.
    DwdIconD2EpsEnsembleMean,
    /// NOAA NCEP: `ncep_gefs_ensemble_mean_seamless`.
    NcepGefsEnsembleMeanSeamless,
    /// NOAA NCEP: `ncep_gefs025_ensemble_mean`.
    NcepGefs025EnsembleMean,
    /// NOAA NCEP: `ncep_gefs05_ensemble_mean`.
    NcepGefs05EnsembleMean,
    /// ECMWF: `ecmwf_ifs025_ensemble_mean`.
    EcmwfIfs025EnsembleMean,
    /// ECMWF: `ecmwf_aifs025_ensemble_mean`.
    EcmwfAifs025EnsembleMean,
    /// Swiss weather service: `meteoswiss_icon_ch1_ensemble_mean`.
    MeteoswissIconCh1EnsembleMean,
    /// Swiss weather service: `meteoswiss_icon_ch2_ensemble_mean`.
    MeteoswissIconCh2EnsembleMean,
    /// Environment Canada: `cmc_gem_geps_ensemble_mean`.
    CmcGemGepsEnsembleMean,
    /// UK Met Office: `ukmo_global_ensemble_mean_20km`.
    UkmoGlobalEnsembleMean20km,
    /// UK Met Office: `ukmo_uk_ensemble_mean_2km`.
    UkmoUkEnsembleMean2km,
}

impl fmt::Display for Model {
    #[allow(clippy::too_many_lines)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
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
    type Error = ConversionError;

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
            _ => Err(ConversionError::InvalidModel {
                model: value.to_string(),
            }),
        }
    }
}

impl From<Model> for String {
    fn from(value: Model) -> Self {
        value.to_string()
    }
}

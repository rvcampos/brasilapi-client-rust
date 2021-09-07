pub const DEFAULT_BASE_URL : &str = "https://brasilapi.com.br/api";

pub(crate) mod banks {
  pub(crate) const SVC_URL: &str = "banks/v1";
  pub(crate) const MIN_CODE: &i16 = &1;
  pub(crate) const MAX_CODE: &i16 = &999;
}

pub(crate) mod cep {
    pub(crate) const SVC_V1_URL: &str = "cep/v1";
    pub(crate) const SVC_V2_URL: &str = "cep/v2";
}

pub(crate) mod ddd {
  pub(crate) const SVC_URL: &str = "ddd/v1";
  pub(crate) const MIN_DDD: &i8 = &11;
  pub(crate) const MAX_DDD: &i8 = &99;
}

pub(crate) mod holidays {
  pub(crate) const SVC_URL: &str = "feriados/v1";
  pub(crate) const MIN_YEAR: &i32 = &1900;
  pub(crate) const MAX_YEAR: &i32 = &2199;
}
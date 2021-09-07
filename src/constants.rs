pub const DEFAULT_BASE_URL : &str = "https://brasilapi.com.br/api";

pub(crate) mod cep {
    pub(crate) const SVC_V1_URL: &str = "cep/v1";
    pub(crate) const SVC_V2_URL: &str = "cep/v2";
}

pub(crate) mod holidays {
  pub(crate) const SVC_URL: &str = "feriados/v1";
  pub(crate) const MIN_YEAR: &i32 = &1900;
  pub(crate) const MAX_YEAR: &i32 = &2199;
}
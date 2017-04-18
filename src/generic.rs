use ::gss_OID;

extern "C" {
    pub static gss_nt_user_name: gss_OID;
    pub static gss_nt_machine_uid_name: gss_OID;
    pub static gss_nt_string_uid_name: gss_OID;
    pub static gss_nt_service_name_v2: gss_OID;
    pub static gss_nt_service_name: gss_OID;
    pub static gss_nt_exported_name: gss_OID;
}

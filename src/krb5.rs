use super::*;
use ::krb5_sys::*;

extern "C" {
    pub static GSS_KRB5_NT_PRINCIPAL_NAME: *const gss_OID_desc;
}

// TODO: defines here

extern "C" {
    pub static gss_mech_krb5: *const gss_OID_desc;
    pub static gss_mech_krb5_old: *const gss_OID_desc;
    pub static gss_mech_krb5_wrong: *const gss_OID_desc;
    pub static gss_mech_iakerb: *const gss_OID_desc;
    pub static gss_mech_set_krb5: *const gss_OID_set_desc;
    pub static gss_mech_set_krb5_old: *const gss_OID_set_desc;
    pub static gss_mech_set_krb5_both: *const gss_OID_set_desc;

    pub static gss_nt_krb5_name: *const gss_OID_desc;
    pub static gss_nt_krb5_principal: *const gss_OID_desc;

    // actually an array but this should work just fine
    pub static krb5_gss_oid_array: *const gss_OID_desc;
}

// TODO: some defines here

#[repr(C)]
pub struct gss_krb5_lucid_key {
    type_: OM_uint32,
    length: OM_uint32,
    data: *mut c_void,
}
pub type gss_krb5_lucid_key_t = gss_krb5_lucid_key;

// TODO: Docs
#[repr(C)]
pub struct gss_krb5_rfc1964_keydata {
    sign_alg: OM_uint32,
    seal_alg: OM_uint32,
    ctx_key: gss_krb5_lucid_key_t,
}
pub type gss_krb5_rfc1964_keydata_t = gss_krb5_rfc1964_keydata;

// TODO: Docs
#[repr(C)]
pub struct gss_krb5_cfx_keydata {
    have_acceptor_subkey: OM_uint32,
    ctx_key: gss_krb5_lucid_key_t,
    acceptor_subkey: gss_krb5_lucid_key_t,
}
pub type gss_krb5_cfx_keydata_t = gss_krb5_cfx_keydata;

// TODO: Docs
#[repr(C)]
pub struct gss_krb5_lucid_context_v1 {
    version: OM_uint32,
    initiate: OM_uint32,
    endtime: OM_uint32,
    send_seq: u64,
    recv_seq: u64,
    protocol: OM_uint32,
    rfc1964_kd: gss_krb5_rfc1964_keydata_t,
    cfx_kd: gss_krb5_cfx_keydata_t,
}
pub type gss_krb5_lucid_context_v1_t = gss_krb5_lucid_context_v1;

// TODO: Docs
#[repr(C)]
pub struct gss_krb5_lucid_context_version {
    version: OM_uint32,
}
pub type gss_krb5_lucid_context_version_t = gss_krb5_lucid_context_version;

// TODO: define here

extern "C" {
    pub fn krb5_gss_register_acceptor_identity(arg: *const c_char) -> OM_uint32;
    pub fn gss_krb5_get_tkt_flags(minor: *mut OM_uint32,
                                  context_handle: gss_ctx_id_t,
                                  ticket_flags: *mut krb5_flags) -> OM_uint32;
    pub fn gss_krb5_copy_ccache(minor: *mut OM_uint32,
                                cred_handle: gss_cred_id_t,
                                out_ccache: krb5_ccache) -> OM_uint32;
    pub fn gss_krb5_ccache_name(minor: *mut OM_uint32,
                                name: *const c_char,
                                out_name: *mut *const c_char) -> OM_uint32;
    pub fn gss_krb5_set_allowable_enctypes(minor: OM_uint32,
                                           cred: gss_cred_id_t,
                                           num_ktypes: OM_uint32,
                                           ktypes: *mut krb5_enctype) -> OM_uint32;
    pub fn gss_krb5_export_lucid_sec_context(minor: OM_uint32,
                                             context_handle: *mut gss_ctx_id_t,
                                             version: OM_uint32,
                                             kctx: *mut *mut c_void) -> OM_uint32;
    pub fn gss_krb5_free_lucid_sec_context(minor: *mut OM_uint32,
                                           kctx: *mut c_void) -> OM_uint32;
    pub fn gsskrb5_extract_authz_data_from_sec_context(minor: *mut OM_uint32,
                                                       context_handle: /* const */ gss_ctx_id_t,
                                                       ad_type: c_int,
                                                       ad_data: gss_buffer_t) -> OM_uint32;
    pub fn gss_krb5_set_cred_rcache(minor: *mut OM_uint32,
                                    cred: gss_cred_id_t,
                                    rcache: krb5_rcache) -> OM_uint32;
    pub fn gsskrb5_extract_authtime_from_sec_context(minor: *mut OM_uint32,
                                                     ctx_handle: gss_ctx_id_t,
                                                     timestamp_out: *mut krb5_timestamp) -> OM_uint32;
    pub fn gss_krb5_import_cred(minor: *mut OM_uint32,
                                id: krb5_ccache,
                                keytab_principal: krb5_principal,
                                keytab: krb5_keytab,
                                cred: *mut gss_cred_id_t) -> OM_uint32;
}

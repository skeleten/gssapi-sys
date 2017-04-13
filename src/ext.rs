use ::std::os::raw::*;
use super::*;

extern "C" {
    pub fn gss_localname(minor: *mut OM_uint32,
                         name: /* const */ gss_name_t,
                         mech_type: gss_const_OID,
                         localname: gss_buffer_t) -> OM_uint32;
    pub fn gss_userok(name: /* const */ gss_name_t,
                      username: *const c_char) -> c_int;
    pub fn gss_authorize_localname(minor: *mut OM_uint32,
                                   name: /* const */ gss_name_t,
                                   user: /* const */ gss_name_t) -> OM_uint32;
    pub fn gss_acquire_cred_with_password(minor_status: *mut OM_uint32,
                                          desired_name: /* const */ gss_name_t,
                                          password: /* const */ gss_buffer_t,
                                          time_req: OM_uint32,
                                          desired_mechs: /* const */ gss_OID_set,
                                          cred_usage: gss_cred_usage_t,
                                          output_cred_handle: gss_cred_id_t,
                                          actual_mechs: *mut gss_OID_set,
                                          time_rec: *mut OM_uint32) -> OM_uint32;
    pub fn gss_add_cred_with_password(minor: *mut OM_uint32,
                                      input_cred_handle: /* const */ gss_cred_id_t,
                                      desired_name: /* const */ gss_name_t,
                                      desired_mech: /* const */ gss_OID,
                                      password: /* const */ gss_buffer_t,
                                      cred_usage: gss_cred_usage_t,
                                      initiator_time_req: OM_uint32,
                                      acceptor_time_req: OM_uint32,
                                      output_cred_handle: *mut gss_cred_id_t,
                                      acual_mechs: *mut gss_OID_set,
                                      initiator_time_rec: *mut OM_uint32,
                                      acceptor_time_rec: *mut OM_uint32) -> OM_uint32;
}

#[repr(C)]
pub struct gss_buffer_set_desc_struct {
    count: usize,
    elements: *mut gss_buffer_desc,
}
pub type gss_buffer_set_desc = gss_buffer_set_desc_struct;
pub type gss_buffer_set_t = *mut gss_buffer_set_desc_struct;

pub const GSS_C_NO_BUFFER_SET: gss_buffer_set_t = 0 as *mut _;

extern "C" {
    pub fn gss_create_empty_buffer_set(minor: *mut OM_uint32,
                                       buffer_set: *mut gss_buffer_set_t) -> OM_uint32;
    pub fn gss_add_buffer_set_member(minor: *mut OM_uint32,
                                     member_buffer: /* const */ gss_buffer_t,
                                     buffer_set: gss_buffer_set_t)  -> OM_uint32;
    pub fn gss_release_buffer_set(minor: *mut OM_uint32,
                                  buffer_set: *mut gss_buffer_set_t) -> OM_uint32;
    pub fn gss_inquire_sec_context_by_oid(minor: *mut OM_uint32,
                                          context_handle: /* const */ gss_ctx_id_t,
                                          desired_object: /* const */ gss_OID,
                                          data_set: *mut gss_buffer_set_t) -> OM_uint32;
    pub fn gss_inquire_cred_by_oid(minor: *mut OM_uint32,
                                   cred_handle: /* const */ gss_cred_id_t,
                                   desired_object: /* const */ gss_OID,
                                   data_set: gss_buffer_set_t) -> OM_uint32;
    pub fn gss_set_sec_context_option(minor: *mut OM_uint32,
                                      cred_handle: *mut gss_ctx_id_t,
                                      desired_object: /* const */ gss_OID,
                                      value: /* const */ gss_buffer_t) -> OM_uint32;
    pub fn gss_set_cred_option(minor: *mut OM_uint32,
                               cred: *mut gss_cred_id_t,
                               desired_object: /* const */ gss_OID,
                               value: /* const */ gss_buffer_t) -> OM_uint32;
    pub fn gssspi_mech_invoke(minor: *mut OM_uint32,
                              desired_mech: /* const */ gss_OID,
                              desired_object: /* const */ gss_OID,
                              value: gss_buffer_t) -> OM_uint32;
}

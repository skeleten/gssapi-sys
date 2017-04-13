#![allow(non_camel_case_types,
         non_upper_case_globals)]

use std::os::raw::*;

pub mod ext;

// TODO: Doc
pub enum gss_name_struct { }
pub type gss_name_t = *mut gss_name_struct;


// TODO: Doc
pub enum gss_cred_id_struct { }
pub type gss_cred_id_t = *mut gss_cred_id_struct;


// TODO: Doc
pub enum gss_ctx_id_struct { }
pub type gss_ctx_id_t = *mut gss_ctx_id_struct;

pub type gss_uint32 = u32;
pub type gss_int32 = i32;


// NOTE: xom.h is needed for these
#[cfg(feature = "om_string")]
pub type gss_OID = *mut OM_object_identifier;
#[cfg(feature = "om_string")]
pub type gss_OID_desc = OM_object_identifier;

#[cfg(not(feature = "om_string"))]
pub type OM_uint32 = gss_uint32;
#[cfg(not(feature = "om_string"))]
#[repr(C)]
pub struct gss_OID_desc_struct {
    length: OM_uint32,
    elements: *mut c_void,
}
#[cfg(not(feature = "om_string"))]
pub type gss_OID_desc = gss_OID_desc_struct;
#[cfg(not(feature = "om_string"))]
pub type gss_OID = *mut gss_OID_desc_struct;

#[repr(C)]
pub struct gss_OID_set_desc_struct {
    count: usize,
    elements: gss_OID,
}
pub type gss_OID_set_desc = gss_OID_set_desc_struct;
pub type gss_OID_set = *mut gss_OID_set_desc_struct;

#[repr(C)]
pub struct gss_buffer_desc_struct {
    length: usize,
    value: *mut c_void,
}
pub type gss_buffer_desc = gss_buffer_desc_struct;
pub type gss_buffer_t = *mut gss_buffer_desc_struct;

#[repr(C)]
pub struct gss_channel_bindings_struct {
    initiator_addrtype: OM_uint32,
    initiator_address: gss_buffer_desc,
    acceptor_addrtype: OM_uint32,
    acceptor_address: gss_buffer_desc,
    application_data: gss_buffer_desc,
}
pub type gss_channel_bindings_t = *mut gss_channel_bindings_struct;

// TODO: Doc
pub type gss_qop_t = OM_uint32;
pub type gss_cred_usage_t = c_int;

pub const GSS_C_DELEG_FLAG: OM_uint32        = 1;
pub const GSS_C_MUTUAL_FLAG: OM_uint32       = 2;
pub const GSS_C_REPLAY_FLAG: OM_uint32       = 4;
pub const GSS_C_SEQUENCE_FLAG: OM_uint32     = 8;
pub const GSS_C_CONF_FLAG: OM_uint32         = 16;
pub const GSS_C_INTEG_FLAG: OM_uint32        = 32;
pub const GSS_C_ANON_FLAG: OM_uint32         = 64;
pub const GSS_C_PROT_READY_FLAG: OM_uint32   = 128;
pub const GSS_C_TRANS_FLAG: OM_uint32        = 256;
pub const GSS_C_DELEG_POLICY_FLAG: OM_uint32 = 32768;

pub const GSS_C_BOTH: OM_uint32     = 0;
pub const GSS_C_INITIATE: OM_uint32 = 1;
pub const GSS_C_ACCEPT: OM_uint32   = 2;

pub const GSS_C_GSS_CODE: OM_uint32  = 1;
pub const GSS_C_MECH_CODE: OM_uint32 = 2;

pub const GSS_C_AF_UNSPEC: OM_uint32    = 0;
pub const GSS_C_AF_LOCAL: OM_uint32     = 1;
pub const GSS_C_AF_INET: OM_uint32      = 2;
pub const GSS_C_AF_IMPLINK: OM_uint32   = 3;
pub const GSS_C_AF_PUP: OM_uint32       = 4;
pub const GSS_C_AF_CHAOS: OM_uint32     = 5;
pub const GSS_C_AF_NS: OM_uint32        = 6;
pub const GSS_C_AF_NBS: OM_uint32       = 7;
pub const GSS_C_AF_ECMA: OM_uint32      = 8;
pub const GSS_C_AF_DATAKIT: OM_uint32   = 9;
pub const GSS_C_AF_CCITT: OM_uint32     = 10;
pub const GSS_C_AF_SNA: OM_uint32       = 11;
pub const GSS_C_AF_DECnet: OM_uint32    = 12;
pub const GSS_C_AF_DLI: OM_uint32       = 13;
pub const GSS_C_AF_LAT: OM_uint32       = 14;
pub const GSS_C_AF_HYLINK: OM_uint32    = 15;
pub const GSS_C_AF_APPLETALK: OM_uint32 = 16;
pub const GSS_C_AF_BSC: OM_uint32       = 17;
pub const GSS_C_AF_DSS: OM_uint32       = 18;
pub const GSS_C_AF_OSI: OM_uint32       = 19;
pub const GSS_C_AF_NETBIOS: OM_uint32   = 20;
pub const GSS_C_AF_X25: OM_uint32       = 21;

pub const GSS_C_AF_NULLADDR: OM_uint32  = 255;

pub const GSS_C_NO_NAME: gss_name_t                         = 0 as *mut _;
pub const GSS_C_NO_BUFFER: gss_buffer_t                     = 0 as *mut _;
pub const GSS_C_NO_OID: gss_OID                             = 0 as *mut _;
pub const GSS_C_NO_OID_SET: gss_OID_set                     = 0 as *mut _;
pub const GSS_C_NO_CONTEXT: gss_ctx_id_t                    = 0 as *mut _;
pub const GSS_C_NO_CREDENTIAL: gss_cred_id_t                = 0 as *mut _;
pub const GSS_C_NO_CHANNEL_BINDINGS: gss_channel_bindings_t = 0 as *mut _;
// TODO: GSS_C_EMPTY_BUFFER  {0, NULL}

pub const GSS_C_NULL_OID: gss_OID = GSS_C_NO_OID;
pub const GSS_C_NULL_OID_SET: gss_OID_set = GSS_C_NO_OID_SET;

pub const GSS_C_QOP_DEFAULT: gss_uint32 = 0;

pub const GSS_C_INDEFINITE: gss_uint32 = 0xffffffff;


pub const GSS_S_COMPLETE: gss_uint32 = 0;

pub const GSS_C_CALLING_ERROR_OFFSET: gss_uint32 = 24;
pub const GSS_C_ROUTINE_ERROR_OFFSET: gss_uint32 = 16;
pub const GSS_C_SUPPLEMENTARY_OFFSET: gss_uint32 = 0;
pub const GSS_C_CALLING_ERROR_MASK: gss_uint32 = 0377;
pub const GSS_C_ROUTINE_ERROR_MASK: gss_uint32 = 0337;
pub const GSS_C_SUPPLEMENTARY_MASK: gss_uint32 = 177777;

// TODO: Macros here

pub const GSS_S_CALL_INACCESSIBLE_READ: gss_uint32 = 1 << GSS_C_CALLING_ERROR_OFFSET;
pub const GSS_S_CALL_INACCESSIBLE_WRITE: gss_uint32 = 2 << GSS_C_CALLING_ERROR_OFFSET;
pub const GSS_S_CALL_BAD_STRUCTURE: gss_uint32 = 3 << GSS_C_CALLING_ERROR_OFFSET;

pub const GSS_S_BAD_MECH: gss_uint32             = (1 << GSS_C_ROUTINE_ERROR_OFFSET);
pub const GSS_S_BAD_NAME: gss_uint32             = (2 << GSS_C_ROUTINE_ERROR_OFFSET);
pub const GSS_S_BAD_NAMETYPE: gss_uint32         = (3 << GSS_C_ROUTINE_ERROR_OFFSET);
pub const GSS_S_BAD_BINDINGS: gss_uint32         = (4 << GSS_C_ROUTINE_ERROR_OFFSET);
pub const GSS_S_BAD_STATUS: gss_uint32           = (5 << GSS_C_ROUTINE_ERROR_OFFSET);
pub const GSS_S_BAD_SIG: gss_uint32              = (6 << GSS_C_ROUTINE_ERROR_OFFSET);
pub const GSS_S_BAD_MIC: gss_uint32              = GSS_S_BAD_SIG;
pub const GSS_S_NO_CRED: gss_uint32              = (7 << GSS_C_ROUTINE_ERROR_OFFSET);
pub const GSS_S_NO_CONTEXT: gss_uint32           = (8 << GSS_C_ROUTINE_ERROR_OFFSET);
pub const GSS_S_DEFECTIVE_TOKEN: gss_uint32      = (9 << GSS_C_ROUTINE_ERROR_OFFSET);
pub const GSS_S_DEFECTIVE_CREDENTIAL: gss_uint32 = (10 << GSS_C_ROUTINE_ERROR_OFFSET);
pub const GSS_S_CREDENTIALS_EXPIRED: gss_uint32  = (11 << GSS_C_ROUTINE_ERROR_OFFSET);
pub const GSS_S_CONTEXT_EXPIRED: gss_uint32      = (12 << GSS_C_ROUTINE_ERROR_OFFSET);
pub const GSS_S_FAILURE: gss_uint32              = (13 << GSS_C_ROUTINE_ERROR_OFFSET);
pub const GSS_S_BAD_QOP: gss_uint32              = (14 << GSS_C_ROUTINE_ERROR_OFFSET);
pub const GSS_S_UNAUTHORIZED: gss_uint32         = (15 << GSS_C_ROUTINE_ERROR_OFFSET);
pub const GSS_S_UNAVAILABLE: gss_uint32          = (16 << GSS_C_ROUTINE_ERROR_OFFSET);
pub const GSS_S_DUPLICATE_ELEMENT: gss_uint32    = (17 << GSS_C_ROUTINE_ERROR_OFFSET);
pub const GSS_S_NAME_NOT_MN: gss_uint32          = (18 << GSS_C_ROUTINE_ERROR_OFFSET);
pub const GSS_S_BAD_MECH_ATTR: gss_uint32        = (19 << GSS_C_ROUTINE_ERROR_OFFSET);

pub const GSS_S_CONTINUE_NEEDED: gss_uint32 = (1 << (GSS_C_SUPPLEMENTARY_OFFSET + 0));
pub const GSS_S_DUPLICATE_TOKEN: gss_uint32 = (1 << (GSS_C_SUPPLEMENTARY_OFFSET + 1));
pub const GSS_S_OLD_TOKEN: gss_uint32       = (1 << (GSS_C_SUPPLEMENTARY_OFFSET + 2));
pub const GSS_S_UNSEQ_TOKEN: gss_uint32     = (1 << (GSS_C_SUPPLEMENTARY_OFFSET + 3));
pub const GSS_S_GAP_TOKEN: gss_uint32       = (1 << (GSS_C_SUPPLEMENTARY_OFFSET + 4));

extern "C" {
    pub static GSS_C_NT_USER_NAME: gss_OID;
    pub static GSS_C_NT_MACHINE_UID_NAME: gss_OID;
    pub static GSS_C_NT_STRING_UID_NAME: gss_OID;
    pub static GSS_C_NT_HOSTBASED_SERVICE_X: gss_OID;
    pub static GSS_C_NT_HOSTBASED_SERVICE: gss_OID;
    pub static GSS_C_NT_ANONYMOUS: gss_OID;
    pub static GSS_C_NT_EXPORT_NAME: gss_OID;

    pub fn gss_acquire_cred(minor_status: *mut OM_uint32,
                            desired_name: gss_name_t,
                            time_req: OM_uint32,
                            desired_mechs: gss_OID_set,
                            cred_usage: gss_cred_usage_t,
                            output_cred_handle: *mut gss_cred_id_t,
                            actual_mechs: *mut gss_OID_set,
                            time_rec: *mut OM_uint32) -> OM_uint32;
    pub fn gss_release_cred(minor_status: *mut OM_uint32,
                            cred_handle: *mut gss_cred_id_t) -> OM_uint32;
    pub fn gss_init_sec_context(minor_status: *mut OM_uint32,
                                claimant_cred_handle: gss_cred_id_t,
                                context_handle: *mut gss_ctx_id_t,
                                target_name: gss_name_t,
                                mech_type: gss_OID,
                                req_flags: OM_uint32,
                                time_eq: OM_uint32,
                                input_chan_bindings: gss_channel_bindings_t,
                                input_token: gss_buffer_t,
                                actual_mech_type: *mut gss_OID,
                                output_token: gss_buffer_t,
                                ret_flags: *mut OM_uint32,
                                time_rec: *mut OM_uint32) -> OM_uint32;
    pub fn gss_accept_sec_context(minor_status: *mut OM_uint32,
                                  context_handle: *mut gss_ctx_id_t,
                                  acceptor_cred_handle: gss_cred_id_t,
                                  input_token_buffer: gss_buffer_t,
                                  input_chan_bindings: gss_channel_bindings_t,
                                  src_name: *mut gss_name_t,
                                  mech_type: *mut gss_OID,
                                  output_token: gss_buffer_t,
                                  ret_flags: *mut OM_uint32,
                                  time_rec: *mut OM_uint32,
                                  delegated_cred_handle: *mut gss_cred_id_t) -> OM_uint32;
    pub fn gss_process_context_token(minor_status: *mut OM_uint32,
                                     context_handle: gss_ctx_id_t,
                                     token_buffer: gss_buffer_t) -> OM_uint32;
    pub fn gss_delete_sec_context(minor_status: *mut OM_uint32,
                                  context_handle: *mut gss_ctx_id_t,
                                  output_token: gss_buffer_t) -> OM_uint32;
    pub fn gss_context_time(minor_status: *mut OM_uint32,
                            context_handle: gss_ctx_id_t,
                            time_rec: *mut OM_uint32) -> OM_uint32;
    pub fn gss_get_mic(minor_status: *mut OM_uint32,
                       context_handle: gss_ctx_id_t,
                       qop_req: gss_qop_t,
                       message_buffer: gss_buffer_t,
                       message_token: gss_buffer_t) -> OM_uint32;
    pub fn gss_verify_mic(minor_mode: *mut OM_uint32,
                          context_handle: gss_ctx_id_t,
                          message_buffer: gss_buffer_t,
                          message_token: gss_buffer_t,
                          qop_state: gss_qop_t) -> OM_uint32;
    pub fn gss_wrap(minor_status: *mut OM_uint32,
                    context_handle: gss_ctx_id_t,
                    conf_req_flag: c_int,
                    qop_req: gss_qop_t,
                    input_message_buffer: gss_buffer_t,
                    conf_state: *mut c_int,
                    output_message_buffer: gss_buffer_t) -> OM_uint32;
    pub fn gss_unwrap(minor_status: *mut OM_uint32,
                      context_handle: gss_ctx_id_t,
                      input_message_buffer: gss_buffer_t,
                      output_message_buffer: gss_buffer_t,
                      conf_state: *mut c_int,
                      qop_state: *mut gss_qop_t) -> OM_uint32;
    pub fn gss_display_status(minor_status: *mut OM_uint32,
                              status_value: OM_uint32,
                              status_type: c_int,
                              mech_type: gss_OID,
                              message_context: *mut OM_uint32,
                              status_string: gss_buffer_t) -> OM_uint32;
    pub fn gss_indicate_mechs(minor_status: *mut OM_uint32,
                              mech_set: gss_OID_set) -> OM_uint32;
    pub fn gss_compare_name(minor_status: *mut OM_uint32,
                            name1: gss_name_t,
                            name2: gss_name_t,
                            name_equal: *mut c_int) -> OM_uint32;
    pub fn gss_display_name(minor_status: *mut OM_uint32,
                            input_name: gss_name_t,
                            output_name_buffer: gss_buffer_t,
                            output_name_type: *mut gss_OID) -> OM_uint32;
    pub fn gss_import_name(minor_status: *mut OM_uint32,
                           input_name_buffer: gss_buffer_t,
                           input_name_type: gss_OID,
                           output_name: gss_name_t) -> OM_uint32;
    pub fn gss_release_name(minor_status: *mut OM_uint32,
                            input_name: gss_name_t) -> OM_uint32;
    pub fn gss_release_buffer(minor_status: *mut OM_uint32,
                              buffer: gss_buffer_t) -> OM_uint32;
    pub fn gss_release_oid_set(minor_status: *mut OM_uint32,
                               set: *mut gss_OID_set) -> OM_uint32;
    pub fn gss_inquire_cred(minor_status: *mut OM_uint32,
                            cred_handle: gss_cred_id_t,
                            name: gss_name_t,
                            lifetime: OM_uint32,
                            cred_usage: gss_cred_usage_t,
                            mechanisms: gss_OID_set) -> OM_uint32;
    pub fn gss_inquire_context(minor_status: *mut OM_uint32,
                               context_handle: gss_ctx_id_t,
                               src_name: *mut gss_name_t,
                               targ_name: *mut gss_name_t,
                               lifetime_src: *mut OM_uint32,
                               mech_type: *mut gss_OID,
                               ctx_flags: *mut OM_uint32,
                               locall_initiated: *mut c_int,
                               open: *mut c_int) -> OM_uint32;
    pub fn gss_wrap_size_limit(minor_status: *mut OM_uint32,
                               context_handle: gss_ctx_id_t,
                               conf_req_flag: c_int,
                               qop_req: gss_qop_t,
                               req_output_size: OM_uint32,
                               max_input_size: *mut OM_uint32) -> OM_uint32;
    pub fn gss_import_name_object(minor_status: *mut OM_uint32,
                                  input_name: *mut c_void,
                                  input_name_type: gss_OID,
                                  output_name: *mut gss_name_t) -> OM_uint32;
    pub fn gss_export_name_object(minor_status: *mut OM_uint32,
                                  input_name: gss_name_t,
                                  desired_name_type: gss_OID,
                                  output_name: *mut *mut c_void) -> OM_uint32;
    pub fn gss_add_cred(minor_status: *mut OM_uint32,
                        input_cred_handle: gss_cred_id_t,
                        desired_name: gss_name_t,
                        desired_mech: gss_OID,
                        cred_usage: gss_cred_usage_t,
                        initiator_time_req: OM_uint32,
                        acceptor_time_req: OM_uint32,
                        output_cred_handle: *mut gss_cred_id_t,
                        actual_mechs: *mut gss_OID_set,
                        initiator_time_rec: *mut OM_uint32,
                        acceptor_time_rec: *mut OM_uint32) -> OM_uint32;
    pub fn gss_inquire_cred_by_mech(minor_status: *mut OM_uint32,
                                    cred_handle: gss_cred_id_t,
                                    mech_type: gss_OID,
                                    name: *mut gss_name_t,
                                    initiator_lifetime: *mut OM_uint32,
                                    acceptor_lifetime: *mut OM_uint32,
                                    cred_usage: *mut gss_cred_usage_t) -> OM_uint32;
    pub fn gss_export_sec_context(minor_status: *mut OM_uint32,
                                  context_handle: *mut gss_ctx_id_t,
                                  interprocess_token: gss_buffer_t) -> OM_uint32;
    pub fn gss_import_sec_context(minor_status: *mut OM_uint32,
                                  interproces_token: gss_buffer_t,
                                  context_handle: *mut gss_ctx_id_t) -> OM_uint32;
    pub fn gss_release_oid(minor_status: *mut OM_uint32,
                           oid: *mut gss_OID) -> OM_uint32;
    pub fn gss_create_empty_oid_set(minor_status: *mut OM_uint32,
                                    oid_set: *mut gss_OID_set) -> OM_uint32;
    pub fn gss_add_oid_set_member(minor_status: *mut OM_uint32,
                                  member_oid: gss_OID,
                                  oid_set: *mut gss_OID_set) -> OM_uint32;
    pub fn gss_test_oid_set_member(minor_status: *mut OM_uint32,
                                   member: gss_OID,
                                   set: gss_OID_set,
                                   present: *mut c_int) -> OM_uint32;
    pub fn gss_str_to_oid(minor_status: *mut OM_uint32,
                          oid_str: gss_buffer_t,
                          oid: *mut gss_OID) -> OM_uint32;
    pub fn gss_oid_to_str(minor_status: *mut OM_uint32,
                          oid: gss_OID,
                          oid_str: gss_buffer_t) -> OM_uint32;
    pub fn gss_inquire_names_for_mech(minor_status: *mut OM_uint32,
                                      mechanism: gss_OID,
                                      name_types: *mut gss_OID_set) -> OM_uint32;
    pub fn gss_inquire_mechs_for_name(minor_status: *mut OM_uint32,
                                      input_name: *const gss_name_struct,
                                      mech_types: *mut gss_OID_set) -> OM_uint32;
    #[deprecated]
    pub fn gss_sign(minor_status: *mut OM_uint32,
                    context_handle: gss_ctx_id_t,
                    qop_req: c_int,
                    message_buffer: gss_buffer_t,
                    message_token: gss_buffer_t) -> OM_uint32;
    #[deprecated]
    pub fn gss_verify(minor_status: *mut OM_uint32,
                      context_handle: gss_ctx_id_t,
                      message_buffer: gss_buffer_t,
                      token_buffer: gss_buffer_t,
                      qop_state: *mut c_int) -> OM_uint32;
    #[deprecated]
    pub fn gss_seal(minor_status: *mut OM_uint32,
                    context_handle: gss_ctx_id_t,
                    conf_req_flag: c_int,
                    qop_req: c_int,
                    input_message_buffer: gss_buffer_t,
                    conf_state: *mut c_int,
                    output_message_buffer: gss_buffer_t) -> OM_uint32;
    #[deprecated]
    pub fn gss_unseal(minor_status: *mut OM_uint32,
                      context_handle: gss_ctx_id_t,
                      input_message_buffer: gss_buffer_t,
                      output_message_buffer: gss_buffer_t,
                      conf_state: *mut c_int,
                      qop_state: *mut c_int) -> OM_uint32;
    pub fn gss_export_name(minor_status: *mut OM_uint32,
                           input_name: *const gss_name_struct,
                           exported_name: gss_buffer_t) -> OM_uint32;
    pub fn gss_duplicate_name(minor_status: *mut OM_uint32,
                              input_name: *const gss_name_struct,
                              dest_name: *mut gss_name_t) -> OM_uint32;
    pub fn gss_canonicalize_name(minor_status: *mut OM_uint32,
                                 input_name: /* const */ gss_name_t,
                                 mech_type: /* const */ gss_OID,
                                 output_name: *mut gss_name_t) -> OM_uint32;
}
// RFC 4401
pub const GSS_C_PRF_KEY_FULL: c_int = 0;
pub const GSS_C_PRF_KEY_PARTIAL: c_int = 1;

extern "C" {
    pub fn gss_pseudo_random(minor_status: *mut OM_uint32,
                             context: gss_ctx_id_t,
                             prf_key: c_int,
                             prf_in: /* const */ gss_buffer_t,
                             desired_output_len: isize,
                             prf_otu: gss_buffer_t) -> OM_uint32;
    pub fn gss_store_cred(minor_status: *mut OM_uint32,
                          input_cred_handle: /* const */ gss_cred_id_t,
                          input_usage: gss_cred_usage_t,
                          desired_mech: /* const */ gss_OID,
                          overwrite_cred: OM_uint32,
                          default_cred: OM_uint32,
                          elements_stored: *mut gss_OID_set,
                          cred_usage_stored: *mut gss_cred_usage_t) -> OM_uint32;
    pub fn gss_set_neg_mechs(minor_status: *mut OM_uint32,
                             cred_handle: gss_cred_id_t,
                             mech_set: /* const */ gss_OID_set) -> OM_uint32;
}

// TODO: some macros here

pub const GSS_S_CRED_UNAVAIL: gss_uint32 = GSS_S_FAILURE;

// RFC 5587

pub type gss_const_buffer_t = *const gss_buffer_desc;
pub type gss_const_channel_bindings_t = *const gss_channel_bindings_struct;
pub type gss_const_ctx_id_t = *const gss_ctx_id_struct;
pub type gss_const_cred_id_t = *const gss_cred_id_struct;
pub type gss_const_name_t = *const gss_name_struct;
pub type gss_const_OID = *const gss_OID_desc;
pub type gss_const_OID_set = *const gss_OID_set_desc;

extern "C" {
    pub fn gss_indicate_mechs_by_attrs(minor_status: *mut OM_uint32,
                                       desired_mech_attrs: gss_const_OID_set,
                                       except_mech_attrs: gss_OID_set,
                                       critical_mech_attrs: gss_const_OID_set,
                                       mechs: *mut gss_OID_set) -> OM_uint32;
    pub fn gss_inquire_attrs_for_mech(minor_status: *mut OM_uint32,
                                      mech: gss_const_OID,
                                      mech_attrs: *mut gss_OID_set,
                                      known_mech_attrs: *mut gss_OID_set) -> OM_uint32;
    pub fn gss_display_mech_attr(minor_status: *mut OM_uint32,
                                 mech_attr: gss_const_OID,
                                 name: gss_buffer_t,
                                 short_desc: gss_buffer_t,
                                 long_desc: gss_buffer_t) -> OM_uint32;

    pub static GSS_C_MA_MECH_CONCRETE: gss_const_OID;
    pub static GSS_C_MA_MECH_PSEUDO: gss_const_OID;
    pub static GSS_C_MA_MECH_COMPOSITE: gss_const_OID;
    pub static GSS_C_MA_MECH_NEGO: gss_const_OID;
    pub static GSS_C_MA_MECH_GLUE: gss_const_OID;
    pub static GSS_C_MA_NOT_MECH: gss_const_OID;
    pub static GSS_C_MA_DEPRECATED: gss_const_OID;
    pub static GSS_C_MA_NOT_DFLT_MECH: gss_const_OID;
    pub static GSS_C_MA_ITOK_FRAMED: gss_const_OID;
    pub static GSS_C_MA_AUTH_INIT: gss_const_OID;
    pub static GSS_C_MA_AUTH_TARG: gss_const_OID;
    pub static GSS_C_MA_AUTH_INIT_INIT: gss_const_OID;
    pub static GSS_C_MA_AUTH_TARG_INIT: gss_const_OID;
    pub static GSS_C_MA_AUTH_INIT_ANON: gss_const_OID;
    pub static GSS_C_MA_AUTH_TARG_ANON: gss_const_OID;
    pub static GSS_C_MA_DELEG_CRED: gss_const_OID;
    pub static GSS_C_MA_INTEG_PROT: gss_const_OID;
    pub static GSS_C_MA_CONF_PROT: gss_const_OID;
    pub static GSS_C_MA_MIC: gss_const_OID;
    pub static GSS_C_MA_WRAP: gss_const_OID;
    pub static GSS_C_MA_PROT_READY: gss_const_OID;
    pub static GSS_C_MA_REPLAY_DET: gss_const_OID;
    pub static GSS_C_MA_OOS_DET: gss_const_OID;
    pub static GSS_C_MA_CBINDINGS: gss_const_OID;
    pub static GSS_C_MA_PFS: gss_const_OID;
    pub static GSS_C_MA_COMPRESS: gss_const_OID;
    pub static GSS_C_MA_CTX_TRANS: gss_const_OID;

    pub fn gss_inquire_saslname_for_mech(minor_status: *mut OM_uint32,
                                         desired_mech: /* const */ gss_OID,
                                         sasl_mech_name: gss_buffer_t,
                                         mech_name: gss_buffer_t,
                                         mech_description: gss_buffer_t) -> OM_uint32;
    pub fn gss_inquire_mech_for_saslname(minor_status: *mut OM_uint32,
                                         sasl_mech_name: /* const */ gss_buffer_t,
                                         mech_type: *mut gss_OID) -> OM_uint32;

}

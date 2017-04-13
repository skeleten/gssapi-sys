use std::os::raw::*;

// TODO: Doc
pub enum gss_name_struct { }
pub type gss_name_t = *mut gss_name_struct;


// TODO: Doc
pub enum gss_cred_id_struct { }
pub type gss_cred_id_t = *mut gss_cred_id_struct;


// TODO: Doc
pub enum gss_ctx_id_struct { }
pub type gss_ctx_id_t = *mut gss_ctx_id_struct;

type gss_uint32 = u32;
type gss_int32 = i32;


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

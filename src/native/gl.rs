#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]

pub type GLenum = ::core::ffi::c_uint;
pub type GLboolean = ::core::ffi::c_uchar;
pub type GLbitfield = ::core::ffi::c_uint;
pub type GLvoid = ::core::ffi::c_void;
pub type GLbyte = ::core::ffi::c_schar;
pub type GLshort = ::core::ffi::c_short;
pub type GLint = ::core::ffi::c_int;
pub type GLubyte = ::core::ffi::c_uchar;
pub type GLushort = ::core::ffi::c_ushort;
pub type GLuint = ::core::ffi::c_uint;
pub type GLuint64 = ::core::ffi::c_ulonglong;
pub type GLsizei = ::core::ffi::c_int;
pub type GLchar = ::core::ffi::c_char;

pub type khronos_ssize_t = ::core::ffi::c_long;
pub type khronos_usize_t = ::core::ffi::c_ulong;
pub type khronos_intptr_t = ::core::ffi::c_long;

pub type GLsizeiptr = khronos_ssize_t;
pub type GLintptr = khronos_intptr_t;

pub type GLfloat = f32;
pub type GLclampf = f32;
pub type GLdouble = f64;
pub type GLclampd = f64;

pub const GL_INT_2_10_10_10_REV: u32 = 0x8D9F;
pub const GL_PROGRAM_POINT_SIZE: u32 = 0x8642;
pub const GL_STENCIL_ATTACHMENT: u32 = 0x8D20;
pub const GL_DEPTH_ATTACHMENT: u32 = 0x8D00;
pub const GL_COLOR_ATTACHMENT2: u32 = 0x8CE2;
pub const GL_COLOR_ATTACHMENT0: u32 = 0x8CE0;
pub const GL_COLOR_ATTACHMENT22: u32 = 0x8CF6;
pub const GL_DRAW_FRAMEBUFFER: u32 = 0x8CA9;
pub const GL_FRAMEBUFFER_COMPLETE: u32 = 0x8CD5;
pub const GL_NUM_EXTENSIONS: u32 = 0x821D;
pub const GL_INFO_LOG_LENGTH: u32 = 0x8B84;
pub const GL_VERTEX_SHADER: u32 = 0x8B31;
pub const GL_INCR: u32 = 0x1E02;
pub const GL_DYNAMIC_DRAW: u32 = 0x88E8;
pub const GL_STATIC_DRAW: u32 = 0x88E4;
pub const GL_TEXTURE_CUBE_MAP_POSITIVE_Z: u32 = 0x8519;
pub const GL_TEXTURE_CUBE_MAP: u32 = 0x8513;
pub const GL_FUNC_SUBTRACT: u32 = 0x800A;
pub const GL_FUNC_REVERSE_SUBTRACT: u32 = 0x800B;
pub const GL_CONSTANT_COLOR: u32 = 0x8001;
pub const GL_DECR_WRAP: u32 = 0x8508;
pub const GL_LINEAR_MIPMAP_LINEAR: u32 = 0x2703;
pub const GL_ELEMENT_ARRAY_BUFFER: u32 = 0x8893;
pub const GL_SHORT: u32 = 0x1402;
pub const GL_DEPTH_TEST: u32 = 0x0B71;
pub const GL_TEXTURE_CUBE_MAP_NEGATIVE_Y: u32 = 0x8518;
pub const GL_LINK_STATUS: u32 = 0x8B82;
pub const GL_TEXTURE_CUBE_MAP_POSITIVE_Y: u32 = 0x8517;
pub const GL_SAMPLE_ALPHA_TO_COVERAGE: u32 = 0x809E;
pub const GL_RGBA16F: u32 = 0x881A;
pub const GL_CONSTANT_ALPHA: u32 = 0x8003;
pub const GL_READ_FRAMEBUFFER: u32 = 0x8CA8;
pub const GL_TEXTURE0: u32 = 0x84C0;
pub const GL_TEXTURE_MIN_LOD: u32 = 0x813A;
pub const GL_CLAMP_TO_EDGE: u32 = 0x812F;
pub const GL_UNSIGNED_SHORT_5_6_5: u32 = 0x8363;
pub const GL_TEXTURE_WRAP_R: u32 = 0x8072;
pub const GL_UNSIGNED_SHORT_5_5_5_1: u32 = 0x8034;
pub const GL_NEAREST_MIPMAP_NEAREST: u32 = 0x2700;
pub const GL_UNSIGNED_SHORT_4_4_4_4: u32 = 0x8033;
pub const GL_SRC_ALPHA_SATURATE: u32 = 0x0308;
pub const GL_STREAM_DRAW: u32 = 0x88E0;
pub const GL_ONE: u32 = 1;
pub const GL_NEAREST_MIPMAP_LINEAR: u32 = 0x2702;
pub const GL_RGB10_A2: u32 = 0x8059;
pub const GL_RGBA8: u32 = 0x8058;
pub const GL_COLOR_ATTACHMENT1: u32 = 0x8CE1;
pub const GL_RGBA4: u32 = 0x8056;
pub const GL_RGB8: u32 = 0x8051;
pub const GL_ARRAY_BUFFER: u32 = 0x8892;
pub const GL_STENCIL: u32 = 0x1802;
pub const GL_TEXTURE_2D: u32 = 0x0DE1;
pub const GL_DEPTH: u32 = 0x1801;
pub const GL_FRONT: u32 = 0x0404;
pub const GL_STENCIL_BUFFER_BIT: u32 = 0x00000400;
pub const GL_REPEAT: u32 = 0x2901;
pub const GL_RGBA: u32 = 0x1908;
pub const GL_TEXTURE_CUBE_MAP_POSITIVE_X: u32 = 0x8515;
pub const GL_DECR: u32 = 0x1E03;
pub const GL_FRAGMENT_SHADER: u32 = 0x8B30;
pub const GL_FLOAT: u32 = 0x1406;
pub const GL_TEXTURE_MAX_LOD: u32 = 0x813B;
pub const GL_DEPTH_COMPONENT: u32 = 0x1902;
pub const GL_ONE_MINUS_DST_ALPHA: u32 = 0x0305;
pub const GL_COLOR: u32 = 0x1800;
pub const GL_TEXTURE_2D_ARRAY: u32 = 0x8C1A;
pub const GL_TRIANGLES: u32 = 0x0004;
pub const GL_UNSIGNED_BYTE: u32 = 0x1401;
pub const GL_TEXTURE_MAG_FILTER: u32 = 0x2800;
pub const GL_ONE_MINUS_CONSTANT_ALPHA: u32 = 0x8004;
pub const GL_NONE: u32 = 0;
pub const GL_SRC_COLOR: u32 = 0x0300;
pub const GL_BYTE: u32 = 0x1400;
pub const GL_TEXTURE_CUBE_MAP_NEGATIVE_Z: u32 = 0x851A;
pub const GL_LINE_STRIP: u32 = 0x0003;
pub const GL_LINE_SMOOTH: u32 = 0x0B20;
pub const GL_LINE_SMOOTH_HINT: u32 = 0x0C52;
pub const GL_NICEST: u32 = 0x1102;
pub const GL_TEXTURE_3D: u32 = 0x806F;
pub const GL_CW: u32 = 0x0900;
pub const GL_LINEAR: u32 = 0x2601;
pub const GL_RENDERBUFFER: u32 = 0x8D41;
pub const GL_GEQUAL: u32 = 0x0206;
pub const GL_COLOR_BUFFER_BIT: u32 = 0x00004000;
pub const GL_RGBA32F: u32 = 0x8814;
pub const GL_BLEND: u32 = 0x0BE2;
pub const GL_ONE_MINUS_SRC_ALPHA: u32 = 0x0303;
pub const GL_ONE_MINUS_CONSTANT_COLOR: u32 = 0x8002;
pub const GL_TEXTURE_WRAP_T: u32 = 0x2803;
pub const GL_TEXTURE_WRAP_S: u32 = 0x2802;
pub const GL_TEXTURE_MIN_FILTER: u32 = 0x2801;
pub const GL_LINEAR_MIPMAP_NEAREST: u32 = 0x2701;
pub const GL_EXTENSIONS: u32 = 0x1F03;
pub const GL_NO_ERROR: u32 = 0;
pub const GL_REPLACE: u32 = 0x1E01;
pub const GL_KEEP: u32 = 0x1E00;
pub const GL_CCW: u32 = 0x0901;
pub const GL_TEXTURE_CUBE_MAP_NEGATIVE_X: u32 = 0x8516;
pub const GL_RGB: u32 = 0x1907;
pub const GL_TRIANGLE_STRIP: u32 = 0x0005;
pub const GL_FALSE: u32 = 0;
pub const GL_ZERO: u32 = 0;
pub const GL_CULL_FACE: u32 = 0x0B44;
pub const GL_INVERT: u32 = 0x150A;
pub const GL_INT: u32 = 0x1404;
pub const GL_UNSIGNED_INT: u32 = 0x1405;
pub const GL_UNSIGNED_SHORT: u32 = 0x1403;
pub const GL_NEAREST: u32 = 0x2600;
pub const GL_SCISSOR_TEST: u32 = 0x0C11;
pub const GL_LEQUAL: u32 = 0x0203;
pub const GL_STENCIL_TEST: u32 = 0x0B90;
pub const GL_DITHER: u32 = 0x0BD0;
pub const GL_DEPTH_COMPONENT16: u32 = 0x81A5;
pub const GL_DEPTH_COMPONENT24: u32 = 0x81A6;
pub const GL_DEPTH_COMPONENT32: u32 = 0x81A7;
pub const GL_EQUAL: u32 = 0x0202;
pub const GL_FRAMEBUFFER: u32 = 0x8D40;
pub const GL_RGB5: u32 = 0x8050;
pub const GL_LINES: u32 = 0x0001;
pub const GL_DEPTH_BUFFER_BIT: u32 = 0x00000100;
pub const GL_SRC_ALPHA: u32 = 0x0302;
pub const GL_INCR_WRAP: u32 = 0x8507;
pub const GL_LESS: u32 = 0x0201;
pub const GL_MULTISAMPLE: u32 = 0x809D;
pub const GL_FRAMEBUFFER_BINDING: u32 = 0x8CA6;
pub const GL_BACK: u32 = 0x0405;
pub const GL_ALWAYS: u32 = 0x0207;
pub const GL_FUNC_ADD: u32 = 0x8006;
pub const GL_ONE_MINUS_DST_COLOR: u32 = 0x0307;
pub const GL_NOTEQUAL: u32 = 0x0205;
pub const GL_DST_COLOR: u32 = 0x0306;
pub const GL_COMPILE_STATUS: u32 = 0x8B81;
pub const GL_RED: u32 = 0x1903;
pub const GL_GREEN: u32 = 6404;
pub const GL_BLUE: u32 = 6405;
pub const GL_ALPHA: u32 = 6406;
pub const GL_LUMINANCE: u32 = 6409;
pub const GL_LUMINANCE_ALPHA: u32 = 6410;
pub const GL_ALPHA_BITS: u32 = 3413;
pub const GL_RED_BITS: u32 = 3410;
pub const GL_GREEN_BITS: u32 = 3411;
pub const GL_BLUE_BITS: u32 = 3412;
pub const GL_INDEX_BITS: u32 = 3409;
pub const GL_SUBPIXEL_BITS: u32 = 3408;
pub const GL_AUX_BUFFERS: u32 = 3072;
pub const GL_READ_BUFFER: u32 = 3074;
pub const GL_DRAW_BUFFER: u32 = 3073;
pub const GL_DOUBLEBUFFER: u32 = 3122;
pub const GL_COLOR_ATTACHMENT3: u32 = 0x8CE3;
pub const GL_DST_ALPHA: u32 = 0x0304;
pub const GL_RGB5_A1: u32 = 0x8057;
pub const GL_GREATER: u32 = 0x0204;
pub const GL_POLYGON_OFFSET_FILL: u32 = 0x8037;
pub const GL_TRUE: u32 = 1;
pub const GL_NEVER: u32 = 0x0200;
pub const GL_POINTS: u32 = 0x0000;
pub const GL_ONE_MINUS_SRC_COLOR: u32 = 0x0301;
pub const GL_MIRRORED_REPEAT: u32 = 0x8370;
pub const GL_MAX_COMBINED_TEXTURE_IMAGE_UNITS: u32 = 0x8B4D;
pub const GL_R11F_G11F_B10F: u32 = 0x8C3A;
pub const GL_UNSIGNED_INT_10F_11F_11F_REV: u32 = 0x8C3B;
pub const GL_RGBA32UI: u32 = 0x8D70;
pub const GL_RGB32UI: u32 = 0x8D71;
pub const GL_RGBA16UI: u32 = 0x8D76;
pub const GL_RGB16UI: u32 = 0x8D77;
pub const GL_RGBA8UI: u32 = 0x8D7C;
pub const GL_RGB8UI: u32 = 0x8D7D;
pub const GL_RGBA32I: u32 = 0x8D82;
pub const GL_RGB32I: u32 = 0x8D83;
pub const GL_RGBA16I: u32 = 0x8D88;
pub const GL_RGB16I: u32 = 0x8D89;
pub const GL_RGBA8I: u32 = 0x8D8E;
pub const GL_RGB8I: u32 = 0x8D8F;
pub const GL_RED_INTEGER: u32 = 0x8D94;
pub const GL_RG: u32 = 0x8227;
pub const GL_RG_INTEGER: u32 = 0x8228;
pub const GL_R8: u32 = 0x8229;
pub const GL_R16: u32 = 0x822A;
pub const GL_RG8: u32 = 0x822B;
pub const GL_RG16: u32 = 0x822C;
pub const GL_R16F: u32 = 0x822D;
pub const GL_R32F: u32 = 0x822E;
pub const GL_RG16F: u32 = 0x822F;
pub const GL_RG32F: u32 = 0x8230;
pub const GL_R8I: u32 = 0x8231;
pub const GL_R8UI: u32 = 0x8232;
pub const GL_R16I: u32 = 0x8233;
pub const GL_R16UI: u32 = 0x8234;
pub const GL_R32I: u32 = 0x8235;
pub const GL_R32UI: u32 = 0x8236;
pub const GL_RG8I: u32 = 0x8237;
pub const GL_RG8UI: u32 = 0x8238;
pub const GL_RG16I: u32 = 0x8239;
pub const GL_RG16UI: u32 = 0x823A;
pub const GL_RG32I: u32 = 0x823B;
pub const GL_RG32UI: u32 = 0x823C;
pub const GL_RGBA_INTEGER: u32 = 0x8D99;
pub const GL_R8_SNORM: u32 = 0x8F94;
pub const GL_RG8_SNORM: u32 = 0x8F95;
pub const GL_RGB8_SNORM: u32 = 0x8F96;
pub const GL_RGBA8_SNORM: u32 = 0x8F97;
pub const GL_R16_SNORM: u32 = 0x8F98;
pub const GL_RG16_SNORM: u32 = 0x8F99;
pub const GL_RGB16_SNORM: u32 = 0x8F9A;
pub const GL_RGBA16_SNORM: u32 = 0x8F9B;
pub const GL_RGBA16: u32 = 0x805B;
pub const GL_MAX_TEXTURE_SIZE: u32 = 0x0D33;
pub const GL_MAX_CUBE_MAP_TEXTURE_SIZE: u32 = 0x851C;
pub const GL_MAX_3D_TEXTURE_SIZE: u32 = 0x8073;
pub const GL_MAX_ARRAY_TEXTURE_LAYERS: u32 = 0x88FF;
pub const GL_MAX_VERTEX_ATTRIBS: u32 = 0x8869;
pub const GL_CLAMP_TO_BORDER: u32 = 0x812D;
pub const GL_TEXTURE_BORDER_COLOR: u32 = 0x1004;
pub const GL_UNPACK_ALIGNMENT: u32 = 3317;
pub const GL_TEXTURE_SWIZZLE_R: u32 = 36418;
pub const GL_TEXTURE_SWIZZLE_G: u32 = 36419;
pub const GL_TEXTURE_SWIZZLE_B: u32 = 36420;
pub const GL_TEXTURE_SWIZZLE_A: u32 = 36421;
pub const GL_TEXTURE_SWIZZLE_RGBA: u32 = 36422;
pub const GL_DRAW_FRAMEBUFFER_BINDING: u32 = 36006;
pub const GL_TIME_ELAPSED: u32 = 35007;
pub const GL_QUERY_RESULT: u32 = 34918;
pub const GL_QUERY_RESULT_AVAILABLE: u32 = 34919;
pub const GL_VENDOR: u32 = 0x1F00;
pub const GL_VERSION: u32 = 0x1F02;
pub const GL_SHADING_LANGUAGE_VERSION: GLenum = 0x8B8C;
pub const GL_FRONT_AND_BACK: GLenum = 0x0408;
pub const GL_FILL: GLenum = 0x1B02;
pub const GL_LINE: GLenum = 0x1B01;
pub const GL_TEXTURE_BASE_LEVEL: GLenum = 0x813C;
pub const GL_TEXTURE_MAX_LEVEL: GLenum = 0x813D;
pub const GL_TEXTURE_CUBE_MAP_SEAMLESS: GLenum = 0x884F;

// Additional OpenGL 1.5 Constants
pub const GL_READ_ONLY: GLenum = 0x88B8;
pub const GL_WRITE_ONLY: GLenum = 0x88B9;
pub const GL_READ_WRITE: GLenum = 0x88BA;

// OpenGL 4.3 Constants
pub const GL_NUM_SHADING_LANGUAGE_VERSIONS: GLenum = 0x82E9;
pub const GL_VERTEX_ATTRIB_ARRAY_LONG: GLenum = 0x874E;
pub const GL_COMPRESSED_RGB8_ETC2: GLenum = 0x9274;
pub const GL_COMPRESSED_SRGB8_ETC2: GLenum = 0x9275;
pub const GL_COMPRESSED_RGB8_PUNCHTHROUGH_ALPHA1_ETC2: GLenum = 0x9276;
pub const GL_COMPRESSED_SRGB8_PUNCHTHROUGH_ALPHA1_ETC2: GLenum = 0x9277;
pub const GL_COMPRESSED_RGBA8_ETC2_EAC: GLenum = 0x9278;
pub const GL_COMPRESSED_SRGB8_ALPHA8_ETC2_EAC: GLenum = 0x9279;
pub const GL_COMPRESSED_R11_EAC: GLenum = 0x9270;
pub const GL_COMPRESSED_SIGNED_R11_EAC: GLenum = 0x9271;
pub const GL_COMPRESSED_RG11_EAC: GLenum = 0x9272;
pub const GL_COMPRESSED_SIGNED_RG11_EAC: GLenum = 0x9273;
pub const GL_PRIMITIVE_RESTART_FIXED_INDEX: GLenum = 0x8D69;
pub const GL_ANY_SAMPLES_PASSED_CONSERVATIVE: GLenum = 0x8D6A;
pub const GL_MAX_ELEMENT_INDEX: GLenum = 0x8D6B;
pub const GL_COMPUTE_SHADER: GLenum = 0x91B9;
pub const GL_MAX_COMPUTE_UNIFORM_BLOCKS: GLenum = 0x91BB;
pub const GL_MAX_COMPUTE_TEXTURE_IMAGE_UNITS: GLenum = 0x91BC;
pub const GL_MAX_COMPUTE_IMAGE_UNIFORMS: GLenum = 0x91BD;
pub const GL_MAX_COMPUTE_SHARED_MEMORY_SIZE: GLenum = 0x8262;
pub const GL_MAX_COMPUTE_UNIFORM_COMPONENTS: GLenum = 0x8263;
pub const GL_MAX_COMPUTE_ATOMIC_COUNTER_BUFFERS: GLenum = 0x8264;
pub const GL_MAX_COMPUTE_ATOMIC_COUNTERS: GLenum = 0x8265;
pub const GL_MAX_COMBINED_COMPUTE_UNIFORM_COMPONENTS: GLenum = 0x8266;
pub const GL_MAX_COMPUTE_WORK_GROUP_INVOCATIONS: GLenum = 0x90EB;
pub const GL_MAX_COMPUTE_WORK_GROUP_COUNT: GLenum = 0x91BE;
pub const GL_MAX_COMPUTE_WORK_GROUP_SIZE: GLenum = 0x91BF;
pub const GL_COMPUTE_WORK_GROUP_SIZE: GLenum = 0x8267;
pub const GL_UNIFORM_BLOCK_REFERENCED_BY_COMPUTE_SHADER: GLenum = 0x90EC;
pub const GL_ATOMIC_COUNTER_BUFFER_REFERENCED_BY_COMPUTE_SHADER: GLenum = 0x90ED;
pub const GL_DISPATCH_INDIRECT_BUFFER: GLenum = 0x90EE;
pub const GL_DISPATCH_INDIRECT_BUFFER_BINDING: GLenum = 0x90EF;
pub const GL_COMPUTE_SHADER_BIT: GLbitfield = 0x00000020;
pub const GL_DEBUG_OUTPUT_SYNCHRONOUS: GLenum = 0x8242;
pub const GL_DEBUG_NEXT_LOGGED_MESSAGE_LENGTH: GLenum = 0x8243;
pub const GL_DEBUG_CALLBACK_FUNCTION: GLenum = 0x8244;
pub const GL_DEBUG_CALLBACK_USER_PARAM: GLenum = 0x8245;
pub const GL_DEBUG_SOURCE_API: GLenum = 0x8246;
pub const GL_DEBUG_SOURCE_WINDOW_SYSTEM: GLenum = 0x8247;
pub const GL_DEBUG_SOURCE_SHADER_COMPILER: GLenum = 0x8248;
pub const GL_DEBUG_SOURCE_THIRD_PARTY: GLenum = 0x8249;
pub const GL_DEBUG_SOURCE_APPLICATION: GLenum = 0x824A;
pub const GL_DEBUG_SOURCE_OTHER: GLenum = 0x824B;
pub const GL_DEBUG_TYPE_ERROR: GLenum = 0x824C;
pub const GL_DEBUG_TYPE_DEPRECATED_BEHAVIOR: GLenum = 0x824D;
pub const GL_DEBUG_TYPE_UNDEFINED_BEHAVIOR: GLenum = 0x824E;
pub const GL_DEBUG_TYPE_PORTABILITY: GLenum = 0x824F;
pub const GL_DEBUG_TYPE_PERFORMANCE: GLenum = 0x8250;
pub const GL_DEBUG_TYPE_OTHER: GLenum = 0x8251;
pub const GL_MAX_DEBUG_MESSAGE_LENGTH: GLenum = 0x9143;
pub const GL_MAX_DEBUG_LOGGED_MESSAGES: GLenum = 0x9144;
pub const GL_DEBUG_LOGGED_MESSAGES: GLenum = 0x9145;
pub const GL_DEBUG_SEVERITY_HIGH: GLenum = 0x9146;
pub const GL_DEBUG_SEVERITY_MEDIUM: GLenum = 0x9147;
pub const GL_DEBUG_SEVERITY_LOW: GLenum = 0x9148;
pub const GL_DEBUG_TYPE_MARKER: GLenum = 0x8268;
pub const GL_DEBUG_TYPE_PUSH_GROUP: GLenum = 0x8269;
pub const GL_DEBUG_TYPE_POP_GROUP: GLenum = 0x826A;
pub const GL_DEBUG_SEVERITY_NOTIFICATION: GLenum = 0x826B;
pub const GL_MAX_DEBUG_GROUP_STACK_DEPTH: GLenum = 0x826C;
pub const GL_DEBUG_GROUP_STACK_DEPTH: GLenum = 0x826D;
pub const GL_BUFFER: GLenum = 0x82E0;
pub const GL_SHADER: GLenum = 0x82E1;
pub const GL_PROGRAM: GLenum = 0x82E2;
pub const GL_QUERY: GLenum = 0x82E3;
pub const GL_PROGRAM_PIPELINE: GLenum = 0x82E4;
pub const GL_SAMPLER: GLenum = 0x82E6;
pub const GL_MAX_LABEL_LENGTH: GLenum = 0x82E8;
pub const GL_DEBUG_OUTPUT: GLenum = 0x92E0;
pub const GL_CONTEXT_FLAG_DEBUG_BIT: GLbitfield = 0x00000002;
pub const GL_MAX_UNIFORM_LOCATIONS: GLenum = 0x826E;
pub const GL_FRAMEBUFFER_DEFAULT_WIDTH: GLenum = 0x9310;
pub const GL_FRAMEBUFFER_DEFAULT_HEIGHT: GLenum = 0x9311;
pub const GL_FRAMEBUFFER_DEFAULT_LAYERS: GLenum = 0x9312;
pub const GL_FRAMEBUFFER_DEFAULT_SAMPLES: GLenum = 0x9313;
pub const GL_FRAMEBUFFER_DEFAULT_FIXED_SAMPLE_LOCATIONS: GLenum = 0x9314;
pub const GL_MAX_FRAMEBUFFER_WIDTH: GLenum = 0x9315;
pub const GL_MAX_FRAMEBUFFER_HEIGHT: GLenum = 0x9316;
pub const GL_MAX_FRAMEBUFFER_LAYERS: GLenum = 0x9317;
pub const GL_MAX_FRAMEBUFFER_SAMPLES: GLenum = 0x9318;
pub const GL_INTERNALFORMAT_SUPPORTED: GLenum = 0x826F;
pub const GL_INTERNALFORMAT_PREFERRED: GLenum = 0x8270;
pub const GL_INTERNALFORMAT_RED_SIZE: GLenum = 0x8271;
pub const GL_INTERNALFORMAT_GREEN_SIZE: GLenum = 0x8272;
pub const GL_INTERNALFORMAT_BLUE_SIZE: GLenum = 0x8273;
pub const GL_INTERNALFORMAT_ALPHA_SIZE: GLenum = 0x8274;
pub const GL_INTERNALFORMAT_DEPTH_SIZE: GLenum = 0x8275;
pub const GL_INTERNALFORMAT_STENCIL_SIZE: GLenum = 0x8276;
pub const GL_INTERNALFORMAT_SHARED_SIZE: GLenum = 0x8277;
pub const GL_INTERNALFORMAT_RED_TYPE: GLenum = 0x8278;
pub const GL_INTERNALFORMAT_GREEN_TYPE: GLenum = 0x8279;
pub const GL_INTERNALFORMAT_BLUE_TYPE: GLenum = 0x827A;
pub const GL_INTERNALFORMAT_ALPHA_TYPE: GLenum = 0x827B;
pub const GL_INTERNALFORMAT_DEPTH_TYPE: GLenum = 0x827C;
pub const GL_INTERNALFORMAT_STENCIL_TYPE: GLenum = 0x827D;
pub const GL_MAX_WIDTH: GLenum = 0x827E;
pub const GL_MAX_HEIGHT: GLenum = 0x827F;
pub const GL_MAX_DEPTH: GLenum = 0x8280;
pub const GL_MAX_LAYERS: GLenum = 0x8281;
pub const GL_MAX_COMBINED_DIMENSIONS: GLenum = 0x8282;
pub const GL_COLOR_COMPONENTS: GLenum = 0x8283;
pub const GL_DEPTH_COMPONENTS: GLenum = 0x8284;
pub const GL_STENCIL_COMPONENTS: GLenum = 0x8285;
pub const GL_COLOR_RENDERABLE: GLenum = 0x8286;
pub const GL_DEPTH_RENDERABLE: GLenum = 0x8287;
pub const GL_STENCIL_RENDERABLE: GLenum = 0x8288;
pub const GL_FRAMEBUFFER_RENDERABLE: GLenum = 0x8289;
pub const GL_FRAMEBUFFER_RENDERABLE_LAYERED: GLenum = 0x828A;
pub const GL_FRAMEBUFFER_BLEND: GLenum = 0x828B;
pub const GL_READ_PIXELS: GLenum = 0x828C;
pub const GL_READ_PIXELS_FORMAT: GLenum = 0x828D;
pub const GL_READ_PIXELS_TYPE: GLenum = 0x828E;
pub const GL_TEXTURE_IMAGE_FORMAT: GLenum = 0x828F;
pub const GL_TEXTURE_IMAGE_TYPE: GLenum = 0x8290;
pub const GL_GET_TEXTURE_IMAGE_FORMAT: GLenum = 0x8291;
pub const GL_GET_TEXTURE_IMAGE_TYPE: GLenum = 0x8292;
pub const GL_MIPMAP: GLenum = 0x8293;
pub const GL_MANUAL_GENERATE_MIPMAP: GLenum = 0x8294;
pub const GL_AUTO_GENERATE_MIPMAP: GLenum = 0x8295;
pub const GL_COLOR_ENCODING: GLenum = 0x8296;
pub const GL_SRGB_READ: GLenum = 0x8297;
pub const GL_SRGB_WRITE: GLenum = 0x8298;
pub const GL_FILTER: GLenum = 0x829A;
pub const GL_VERTEX_TEXTURE: GLenum = 0x829B;
pub const GL_TESS_CONTROL_TEXTURE: GLenum = 0x829C;
pub const GL_TESS_EVALUATION_TEXTURE: GLenum = 0x829D;
pub const GL_GEOMETRY_TEXTURE: GLenum = 0x829E;
pub const GL_FRAGMENT_TEXTURE: GLenum = 0x829F;
pub const GL_COMPUTE_TEXTURE: GLenum = 0x82A0;
pub const GL_TEXTURE_SHADOW: GLenum = 0x82A1;
pub const GL_TEXTURE_GATHER: GLenum = 0x82A2;
pub const GL_TEXTURE_GATHER_SHADOW: GLenum = 0x82A3;
pub const GL_SHADER_IMAGE_LOAD: GLenum = 0x82A4;
pub const GL_SHADER_IMAGE_STORE: GLenum = 0x82A5;
pub const GL_SHADER_IMAGE_ATOMIC: GLenum = 0x82A6;
pub const GL_IMAGE_TEXEL_SIZE: GLenum = 0x82A7;
pub const GL_IMAGE_COMPATIBILITY_CLASS: GLenum = 0x82A8;
pub const GL_IMAGE_PIXEL_FORMAT: GLenum = 0x82A9;
pub const GL_IMAGE_PIXEL_TYPE: GLenum = 0x82AA;
pub const GL_SIMULTANEOUS_TEXTURE_AND_DEPTH_TEST: GLenum = 0x82AC;
pub const GL_SIMULTANEOUS_TEXTURE_AND_STENCIL_TEST: GLenum = 0x82AD;
pub const GL_SIMULTANEOUS_TEXTURE_AND_DEPTH_WRITE: GLenum = 0x82AE;
pub const GL_SIMULTANEOUS_TEXTURE_AND_STENCIL_WRITE: GLenum = 0x82AF;
pub const GL_TEXTURE_COMPRESSED_BLOCK_WIDTH: GLenum = 0x82B1;
pub const GL_TEXTURE_COMPRESSED_BLOCK_HEIGHT: GLenum = 0x82B2;
pub const GL_TEXTURE_COMPRESSED_BLOCK_SIZE: GLenum = 0x82B3;
pub const GL_CLEAR_BUFFER: GLenum = 0x82B4;
pub const GL_TEXTURE_VIEW: GLenum = 0x82B5;
pub const GL_VIEW_COMPATIBILITY_CLASS: GLenum = 0x82B6;
pub const GL_FULL_SUPPORT: GLenum = 0x82B7;
pub const GL_CAVEAT_SUPPORT: GLenum = 0x82B8;
pub const GL_IMAGE_CLASS_4_X_32: GLenum = 0x82B9;
pub const GL_IMAGE_CLASS_2_X_32: GLenum = 0x82BA;
pub const GL_IMAGE_CLASS_1_X_32: GLenum = 0x82BB;
pub const GL_IMAGE_CLASS_4_X_16: GLenum = 0x82BC;
pub const GL_IMAGE_CLASS_2_X_16: GLenum = 0x82BD;
pub const GL_IMAGE_CLASS_1_X_16: GLenum = 0x82BE;
pub const GL_IMAGE_CLASS_4_X_8: GLenum = 0x82BF;
pub const GL_IMAGE_CLASS_2_X_8: GLenum = 0x82C0;
pub const GL_IMAGE_CLASS_1_X_8: GLenum = 0x82C1;
pub const GL_IMAGE_CLASS_11_11_10: GLenum = 0x82C2;
pub const GL_IMAGE_CLASS_10_10_10_2: GLenum = 0x82C3;
pub const GL_VIEW_CLASS_128_BITS: GLenum = 0x82C4;
pub const GL_VIEW_CLASS_96_BITS: GLenum = 0x82C5;
pub const GL_VIEW_CLASS_64_BITS: GLenum = 0x82C6;
pub const GL_VIEW_CLASS_48_BITS: GLenum = 0x82C7;
pub const GL_VIEW_CLASS_32_BITS: GLenum = 0x82C8;
pub const GL_VIEW_CLASS_24_BITS: GLenum = 0x82C9;
pub const GL_VIEW_CLASS_16_BITS: GLenum = 0x82CA;
pub const GL_VIEW_CLASS_8_BITS: GLenum = 0x82CB;
pub const GL_VIEW_CLASS_S3TC_DXT1_RGB: GLenum = 0x82CC;
pub const GL_VIEW_CLASS_S3TC_DXT1_RGBA: GLenum = 0x82CD;
pub const GL_VIEW_CLASS_S3TC_DXT3_RGBA: GLenum = 0x82CE;
pub const GL_VIEW_CLASS_S3TC_DXT5_RGBA: GLenum = 0x82CF;
pub const GL_VIEW_CLASS_RGTC1_RED: GLenum = 0x82D0;
pub const GL_VIEW_CLASS_RGTC2_RG: GLenum = 0x82D1;
pub const GL_VIEW_CLASS_BPTC_UNORM: GLenum = 0x82D2;
pub const GL_VIEW_CLASS_BPTC_FLOAT: GLenum = 0x82D3;
pub const GL_UNIFORM: GLenum = 0x92E1;
pub const GL_UNIFORM_BLOCK: GLenum = 0x92E2;
pub const GL_PROGRAM_INPUT: GLenum = 0x92E3;
pub const GL_PROGRAM_OUTPUT: GLenum = 0x92E4;
pub const GL_BUFFER_VARIABLE: GLenum = 0x92E5;
pub const GL_SHADER_STORAGE_BLOCK: GLenum = 0x92E6;
pub const GL_VERTEX_SUBROUTINE: GLenum = 0x92E8;
pub const GL_TESS_CONTROL_SUBROUTINE: GLenum = 0x92E9;
pub const GL_TESS_EVALUATION_SUBROUTINE: GLenum = 0x92EA;
pub const GL_GEOMETRY_SUBROUTINE: GLenum = 0x92EB;
pub const GL_FRAGMENT_SUBROUTINE: GLenum = 0x92EC;
pub const GL_COMPUTE_SUBROUTINE: GLenum = 0x92ED;
pub const GL_VERTEX_SUBROUTINE_UNIFORM: GLenum = 0x92EE;
pub const GL_TESS_CONTROL_SUBROUTINE_UNIFORM: GLenum = 0x92EF;
pub const GL_TESS_EVALUATION_SUBROUTINE_UNIFORM: GLenum = 0x92F0;
pub const GL_GEOMETRY_SUBROUTINE_UNIFORM: GLenum = 0x92F1;
pub const GL_FRAGMENT_SUBROUTINE_UNIFORM: GLenum = 0x92F2;
pub const GL_COMPUTE_SUBROUTINE_UNIFORM: GLenum = 0x92F3;
pub const GL_TRANSFORM_FEEDBACK_VARYING: GLenum = 0x92F4;
pub const GL_ACTIVE_RESOURCES: GLenum = 0x92F5;
pub const GL_MAX_NAME_LENGTH: GLenum = 0x92F6;
pub const GL_MAX_NUM_ACTIVE_VARIABLES: GLenum = 0x92F7;
pub const GL_MAX_NUM_COMPATIBLE_SUBROUTINES: GLenum = 0x92F8;
pub const GL_NAME_LENGTH: GLenum = 0x92F9;
pub const GL_TYPE: GLenum = 0x92FA;
pub const GL_ARRAY_SIZE: GLenum = 0x92FB;
pub const GL_OFFSET: GLenum = 0x92FC;
pub const GL_BLOCK_INDEX: GLenum = 0x92FD;
pub const GL_ARRAY_STRIDE: GLenum = 0x92FE;
pub const GL_MATRIX_STRIDE: GLenum = 0x92FF;
pub const GL_IS_ROW_MAJOR: GLenum = 0x9300;
pub const GL_ATOMIC_COUNTER_BUFFER_INDEX: GLenum = 0x9301;
pub const GL_BUFFER_BINDING: GLenum = 0x9302;
pub const GL_BUFFER_DATA_SIZE: GLenum = 0x9303;
pub const GL_NUM_ACTIVE_VARIABLES: GLenum = 0x9304;
pub const GL_ACTIVE_VARIABLES: GLenum = 0x9305;
pub const GL_REFERENCED_BY_VERTEX_SHADER: GLenum = 0x9306;
pub const GL_REFERENCED_BY_TESS_CONTROL_SHADER: GLenum = 0x9307;
pub const GL_REFERENCED_BY_TESS_EVALUATION_SHADER: GLenum = 0x9308;
pub const GL_REFERENCED_BY_GEOMETRY_SHADER: GLenum = 0x9309;
pub const GL_REFERENCED_BY_FRAGMENT_SHADER: GLenum = 0x930A;
pub const GL_REFERENCED_BY_COMPUTE_SHADER: GLenum = 0x930B;
pub const GL_TOP_LEVEL_ARRAY_SIZE: GLenum = 0x930C;
pub const GL_TOP_LEVEL_ARRAY_STRIDE: GLenum = 0x930D;
pub const GL_LOCATION: GLenum = 0x930E;
pub const GL_LOCATION_INDEX: GLenum = 0x930F;
pub const GL_IS_PER_PATCH: GLenum = 0x92E7;
pub const GL_SHADER_STORAGE_BUFFER: GLenum = 0x90D2;
pub const GL_SHADER_STORAGE_BUFFER_BINDING: GLenum = 0x90D3;
pub const GL_SHADER_STORAGE_BUFFER_START: GLenum = 0x90D4;
pub const GL_SHADER_STORAGE_BUFFER_SIZE: GLenum = 0x90D5;
pub const GL_MAX_VERTEX_SHADER_STORAGE_BLOCKS: GLenum = 0x90D6;
pub const GL_MAX_GEOMETRY_SHADER_STORAGE_BLOCKS: GLenum = 0x90D7;
pub const GL_MAX_TESS_CONTROL_SHADER_STORAGE_BLOCKS: GLenum = 0x90D8;
pub const GL_MAX_TESS_EVALUATION_SHADER_STORAGE_BLOCKS: GLenum = 0x90D9;
pub const GL_MAX_FRAGMENT_SHADER_STORAGE_BLOCKS: GLenum = 0x90DA;
pub const GL_MAX_COMPUTE_SHADER_STORAGE_BLOCKS: GLenum = 0x90DB;
pub const GL_MAX_COMBINED_SHADER_STORAGE_BLOCKS: GLenum = 0x90DC;
pub const GL_MAX_SHADER_STORAGE_BUFFER_BINDINGS: GLenum = 0x90DD;
pub const GL_MAX_SHADER_STORAGE_BLOCK_SIZE: GLenum = 0x90DE;
pub const GL_SHADER_STORAGE_BUFFER_OFFSET_ALIGNMENT: GLenum = 0x90DF;
pub const GL_SHADER_STORAGE_BARRIER_BIT: GLbitfield = 0x00002000;
pub const GL_MAX_COMBINED_SHADER_OUTPUT_RESOURCES: GLenum = 0x8F39;
pub const GL_DEPTH_STENCIL_TEXTURE_MODE: GLenum = 0x90EA;
pub const GL_TEXTURE_BUFFER_OFFSET: GLenum = 0x919D;
pub const GL_TEXTURE_BUFFER_SIZE: GLenum = 0x919E;
pub const GL_TEXTURE_BUFFER_OFFSET_ALIGNMENT: GLenum = 0x919F;
pub const GL_TEXTURE_VIEW_MIN_LEVEL: GLenum = 0x82DB;
pub const GL_TEXTURE_VIEW_NUM_LEVELS: GLenum = 0x82DC;
pub const GL_TEXTURE_VIEW_MIN_LAYER: GLenum = 0x82DD;
pub const GL_TEXTURE_VIEW_NUM_LAYERS: GLenum = 0x82DE;
pub const GL_TEXTURE_IMMUTABLE_LEVELS: GLenum = 0x82DF;
pub const GL_VERTEX_ATTRIB_BINDING: GLenum = 0x82D4;
pub const GL_VERTEX_ATTRIB_RELATIVE_OFFSET: GLenum = 0x82D5;
pub const GL_VERTEX_BINDING_DIVISOR: GLenum = 0x82D6;
pub const GL_VERTEX_BINDING_OFFSET: GLenum = 0x82D7;
pub const GL_VERTEX_BINDING_STRIDE: GLenum = 0x82D8;
pub const GL_MAX_VERTEX_ATTRIB_RELATIVE_OFFSET: GLenum = 0x82D9;
pub const GL_MAX_VERTEX_ATTRIB_BINDINGS: GLenum = 0x82DA;
pub const GL_VERTEX_BINDING_BUFFER: GLenum = 0x8F4F;

pub const GL_DISPLAY_LIST: GLenum = 0x82E7;

pub const WGL_NUMBER_PIXEL_FORMATS_ARB: u32 = 0x2000;
pub const WGL_SUPPORT_OPENGL_ARB: u32 = 0x2010;
pub const WGL_DRAW_TO_WINDOW_ARB: u32 = 0x2001;
pub const WGL_PIXEL_TYPE_ARB: u32 = 0x2013;
pub const WGL_TYPE_RGBA_ARB: u32 = 0x202b;
pub const WGL_ACCELERATION_ARB: u32 = 0x2003;
pub const WGL_NO_ACCELERATION_ARB: u32 = 0x2025;
pub const WGL_RED_BITS_ARB: u32 = 0x2015;
pub const WGL_RED_SHIFT_ARB: u32 = 0x2016;
pub const WGL_GREEN_BITS_ARB: u32 = 0x2017;
pub const WGL_GREEN_SHIFT_ARB: u32 = 0x2018;
pub const WGL_BLUE_BITS_ARB: u32 = 0x2019;
pub const WGL_BLUE_SHIFT_ARB: u32 = 0x201a;
pub const WGL_ALPHA_BITS_ARB: u32 = 0x201b;
pub const WGL_ALPHA_SHIFT_ARB: u32 = 0x201c;
pub const WGL_ACCUM_BITS_ARB: u32 = 0x201d;
pub const WGL_ACCUM_RED_BITS_ARB: u32 = 0x201e;
pub const WGL_ACCUM_GREEN_BITS_ARB: u32 = 0x201f;
pub const WGL_ACCUM_BLUE_BITS_ARB: u32 = 0x2020;
pub const WGL_ACCUM_ALPHA_BITS_ARB: u32 = 0x2021;
pub const WGL_DEPTH_BITS_ARB: u32 = 0x2022;
pub const WGL_STENCIL_BITS_ARB: u32 = 0x2023;
pub const WGL_AUX_BUFFERS_ARB: u32 = 0x2024;
pub const WGL_STEREO_ARB: u32 = 0x2012;
pub const WGL_DOUBLE_BUFFER_ARB: u32 = 0x2011;
pub const WGL_SAMPLES_ARB: u32 = 0x2042;
pub const WGL_FRAMEBUFFER_SRGB_CAPABLE_ARB: u32 = 0x20a9;
pub const WGL_CONTEXT_DEBUG_BIT_ARB: u32 = 0x00000001;
pub const WGL_CONTEXT_FORWARD_COMPATIBLE_BIT_ARB: u32 = 0x00000002;
pub const WGL_CONTEXT_PROFILE_MASK_ARB: u32 = 0x9126;
pub const WGL_CONTEXT_CORE_PROFILE_BIT_ARB: u32 = 0x00000001;
pub const WGL_CONTEXT_COMPATIBILITY_PROFILE_BIT_ARB: u32 = 0x00000002;
pub const WGL_CONTEXT_MAJOR_VERSION_ARB: u32 = 0x2091;
pub const WGL_CONTEXT_MINOR_VERSION_ARB: u32 = 0x2092;
pub const WGL_CONTEXT_FLAGS_ARB: u32 = 0x2094;
pub const WGL_CONTEXT_ROBUST_ACCESS_BIT_ARB: u32 = 0x00000004;
pub const WGL_LOSE_CONTEXT_ON_RESET_ARB: u32 = 0x8252;
pub const WGL_CONTEXT_RESET_NOTIFICATION_STRATEGY_ARB: u32 = 0x8256;
pub const WGL_NO_RESET_NOTIFICATION_ARB: u32 = 0x8261;
pub const WGL_CONTEXT_RELEASE_BEHAVIOR_ARB: u32 = 0x2097;
pub const WGL_CONTEXT_RELEASE_BEHAVIOR_NONE_ARB: u32 = 0;
pub const WGL_CONTEXT_RELEASE_BEHAVIOR_FLUSH_ARB: u32 = 0x2098;
pub const WGL_COLORSPACE_EXT: u32 = 0x309d;
pub const WGL_COLORSPACE_SRGB_EXT: u32 = 0x3089;
pub const ERROR_INVALID_VERSION_ARB: u32 = 0x2095;
pub const ERROR_INVALID_PROFILE_ARB: u32 = 0x2096;
pub const ERROR_INCOMPATIBLE_DEVICE_CONTEXTS_ARB: u32 = 0x2054;

macro_rules! gl_loader {
    (
        $(
            fn $fn:ident ( $($arg:ident : $t:ty),* ) -> $res:ty
        ),*
    ) => {
        mod __pfns {
            use super::*;

            $(
                pub static mut $fn: Option<extern "C" fn ($($arg: $t),*) -> $res> = None;
            )*
        }

        $(
            #[allow(clippy::too_many_arguments)]
            pub unsafe fn $fn($($arg: $t),*) -> $res {
                __pfns::$fn.unwrap()( $($arg),* )
            }
        )*

        pub fn load_gl_funcs<T: FnMut(&str) -> Option<unsafe extern "C" fn() -> ()>>(mut getprocaddr: T) {
            $(
                unsafe {
                    let fn_name = stringify!($fn);
                    __pfns::$fn = ::std::mem::transmute_copy(&getprocaddr(fn_name));
                }
            )*
        }
    };
}

gl_loader!(
    fn glGetStringi(name: GLenum, index: GLuint) -> *const GLubyte,
    fn glGetString(name: GLenum) -> *const GLubyte,
    fn glFramebufferTextureLayer(
        target: GLenum,
        attachment: GLenum,
        texture: GLuint,
        level: GLint,
        layer: GLint
    ) -> (),
    fn glGenFramebuffers(n: GLsizei, framebuffers: *mut GLuint) -> (),
    fn glBindFramebuffer(target: GLenum, framebuffer: GLuint) -> (),
    fn glBindRenderbuffer(target: GLenum, renderbuffer: GLuint) -> (),
    fn glClearBufferfi(buffer: GLenum, drawbuffer: GLint, depth: GLfloat, stencil: GLint) -> (),
    fn glClearBufferfv(buffer: GLenum, drawbuffer: GLint, value: *const GLfloat) -> (),
    fn glClearBufferuiv(buffer: GLenum, drawbuffer: GLint, value: *const GLuint) -> (),
    fn glDeleteRenderbuffers(n: GLsizei, renderbuffers: *const GLuint) -> (),
    fn glUniform4fv(location: GLint, count: GLsizei, value: *const GLfloat) -> (),
    fn glUniform3fv(location: GLint, count: GLsizei, value: *const GLfloat) -> (),
    fn glUniform2fv(location: GLint, count: GLsizei, value: *const GLfloat) -> (),
    fn glUniform1fv(location: GLint, count: GLsizei, value: *const GLfloat) -> (),
    fn glUniform1iv(location: GLint, count: GLsizei, value: *const GLint) -> (),
    fn glUniform2iv(location: GLint, count: GLsizei, value: *const GLint) -> (),
    fn glUniform3iv(location: GLint, count: GLsizei, value: *const GLint) -> (),
    fn glUniform4iv(location: GLint, count: GLsizei, value: *const GLint) -> (),
    fn glUniform1i(location: GLint, v0: GLint) -> (),
    fn glUniform2i(location: GLint, v0: GLint, v1: GLint) -> (),
    fn glUniform3i(location: GLint, v0: GLint, v1: GLint, v2: GLint) -> (),
    fn glUniform4i(location: GLint, v0: GLint, v1: GLint, v2: GLint, v3: GLint) -> (),
    fn glUniform1f(location: GLint, v0: GLfloat) -> (),
    fn glUniform2f(location: GLint, v0: GLfloat, v1: GLfloat) -> (),
    fn glUniform3f(location: GLint, v0: GLfloat, v1: GLfloat, v2: GLfloat) -> (),
    fn glUniform4f(location: GLint, v0: GLfloat, v1: GLfloat, v2: GLfloat, v3: GLfloat) -> (),
    fn glUseProgram(program: GLuint) -> (),
    fn glShaderSource(
        shader: GLuint,
        count: GLsizei,
        string: *const *const GLchar,
        length: *const GLint
    ) -> (),
    fn glLinkProgram(program: GLuint) -> (),
    fn glPixelStorei(pname: GLenum, param: GLint) -> (),
    fn glGetUniformLocation(program: GLuint, name: *const GLchar) -> GLint,
    fn glGetShaderiv(shader: GLuint, pname: GLenum, params: *mut GLint) -> (),
    fn glGetProgramInfoLog(
        program: GLuint,
        bufSize: GLsizei,
        length: *mut GLsizei,
        infoLog: *mut GLchar
    ) -> (),
    fn glGetAttribLocation(program: GLuint, name: *const GLchar) -> GLint,
    fn glDisableVertexAttribArray(index: GLuint) -> (),
    fn glDeleteShader(shader: GLuint) -> (),
    fn glDeleteProgram(program: GLuint) -> (),
    fn glCompileShader(shader: GLuint) -> (),
    fn glStencilFuncSeparate(face: GLenum, func: GLenum, ref_: GLint, mask: GLuint) -> (),
    fn glStencilOpSeparate(face: GLenum, sfail: GLenum, dpfail: GLenum, dppass: GLenum) -> (),
    fn glRenderbufferStorageMultisample(
        target: GLenum,
        samples: GLsizei,
        internalformat: GLenum,
        width: GLsizei,
        height: GLsizei
    ) -> (),
    fn glDrawBuffers(n: GLsizei, bufs: *const GLenum) -> (),
    fn glVertexAttribDivisor(index: GLuint, divisor: GLuint) -> (),
    fn glBufferSubData(
        target: GLenum,
        offset: GLintptr,
        size: GLsizeiptr,
        data: *const ::core::ffi::c_void
    ) -> (),
    fn glGenBuffers(n: GLsizei, buffers: *mut GLuint) -> (),
    fn glCheckFramebufferStatus(target: GLenum) -> GLenum,
    fn glFramebufferRenderbuffer(
        target: GLenum,
        attachment: GLenum,
        renderbuffertarget: GLenum,
        renderbuffer: GLuint
    ) -> (),
    fn glCompressedTexImage2D(
        target: GLenum,
        level: GLint,
        internalformat: GLenum,
        width: GLsizei,
        height: GLsizei,
        border: GLint,
        imageSize: GLsizei,
        data: *const GLvoid
    ) -> (),
    fn glCompressedTexImage3D(
        target: GLenum,
        level: GLint,
        internalformat: GLenum,
        width: GLsizei,
        height: GLsizei,
        depth: GLsizei,
        border: GLint,
        imageSize: GLsizei,
        data: *const GLvoid
    ) -> (),
    fn glActiveTexture(texture: GLenum) -> (),
    fn glTexSubImage3D(
        target: GLenum,
        level: GLint,
        xoffset: GLint,
        yoffset: GLint,
        zoffset: GLint,
        width: GLsizei,
        height: GLsizei,
        depth: GLsizei,
        format: GLenum,
        type_: GLenum,
        pixels: *const GLvoid
    ) -> (),
    fn glUniformMatrix2fv(
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: *const GLfloat
    ) -> (),
    fn glUniformMatrix3fv(
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: *const GLfloat
    ) -> (),
    fn glUniformMatrix4fv(
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: *const GLfloat
    ) -> (),
    fn glRenderbufferStorage(
        target: GLenum,
        internalformat: GLenum,
        width: GLsizei,
        height: GLsizei
    ) -> (),
    fn glPolygonOffset(factor: GLfloat, units: GLfloat) -> (),
    fn glDrawElements(mode: GLenum, count: GLsizei, type_: GLenum, indices: *const GLvoid) -> (),
    fn glDeleteFramebuffers(n: GLsizei, framebuffers: *const GLuint) -> (),
    fn glBlendEquationSeparate(modeRGB: GLenum, modeAlpha: GLenum) -> (),
    fn glDeleteTextures(n: GLsizei, textures: *const GLuint) -> (),
    fn glGetProgramiv(program: GLuint, pname: GLenum, params: *mut GLint) -> (),
    fn glBindTexture(target: GLenum, texture: GLuint) -> (),
    fn glTexImage3D(
        target: GLenum,
        level: GLint,
        internalFormat: GLint,
        width: GLsizei,
        height: GLsizei,
        depth: GLsizei,
        border: GLint,
        format: GLenum,
        type_: GLenum,
        pixels: *const GLvoid
    ) -> (),
    fn glCreateShader(type_: GLenum) -> GLuint,
    fn glTexSubImage2D(
        target: GLenum,
        level: GLint,
        xoffset: GLint,
        yoffset: GLint,
        width: GLsizei,
        height: GLsizei,
        format: GLenum,
        type_: GLenum,
        pixels: *const GLvoid
    ) -> (),
    fn glCopyTexImage2D(
        target: GLenum,
        level: GLint,
        internalformat: GLenum,
        x: GLint,
        y: GLint,
        width: GLsizei,
        height: GLsizei,
        border: GLint
    ) -> (),
    fn glClearDepthf(d: GLfloat) -> (),
    fn glClearDepth(depth: GLclampd) -> (),
    fn glFramebufferTexture2D(
        target: GLenum,
        attachment: GLenum,
        textarget: GLenum,
        texture: GLuint,
        level: GLint
    ) -> (),
    fn glCreateProgram() -> GLuint,
    fn glViewport(x: GLint, y: GLint, width: GLsizei, height: GLsizei) -> (),
    fn glDeleteBuffers(n: GLsizei, buffers: *const GLuint) -> (),
    fn glDrawArrays(mode: GLenum, first: GLint, count: GLsizei) -> (),
    fn glDrawElementsInstanced(
        mode: GLenum,
        count: GLsizei,
        type_: GLenum,
        indices: *const ::core::ffi::c_void,
        instancecount: GLsizei
    ) -> (),
    fn glVertexAttribPointer(
        index: GLuint,
        size: GLint,
        type_: GLenum,
        normalized: GLboolean,
        stride: GLsizei,
        pointer: *const ::core::ffi::c_void
    ) -> (),
    fn glVertexAttribIPointer(
        index: GLuint,
        size: GLint,
        type_: GLenum,
        stride: GLsizei,
        pointer: *const ::core::ffi::c_void
    ) -> (),
    fn glDisable(cap: GLenum) -> (),
    fn glColorMask(red: GLboolean, green: GLboolean, blue: GLboolean, alpha: GLboolean) -> (),
    fn glBindBuffer(target: GLenum, buffer: GLuint) -> (),
    fn glBindVertexArray(array: GLuint) -> (),
    fn glDeleteVertexArrays(n: GLsizei, arrays: *const GLuint) -> (),
    fn glDepthMask(flag: GLboolean) -> (),
    fn glDrawArraysInstanced(
        mode: GLenum,
        first: GLint,
        count: GLsizei,
        instancecount: GLsizei
    ) -> (),
    fn glClearStencil(s: GLint) -> (),
    fn glScissor(x: GLint, y: GLint, width: GLsizei, height: GLsizei) -> (),
    fn glGenRenderbuffers(n: GLsizei, renderbuffers: *mut GLuint) -> (),
    fn glBufferData(
        target: GLenum,
        size: GLsizeiptr,
        data: *const ::core::ffi::c_void,
        usage: GLenum
    ) -> (),
    fn glBlendFuncSeparate(
        sfactorRGB: GLenum,
        dfactorRGB: GLenum,
        sfactorAlpha: GLenum,
        dfactorAlpha: GLenum
    ) -> (),
    fn glGenerateMipmap(target: GLenum) -> (),
    fn glTexParameteri(target: GLenum, pname: GLenum, param: GLint) -> (),
    fn glGetIntegerv(pname: GLenum, params: *mut GLint) -> (),
    fn glEnable(cap: GLenum) -> (),
    fn glBlitFramebuffer(
        srcX0: GLint,
        srcY0: GLint,
        srcX1: GLint,
        srcY1: GLint,
        dstX0: GLint,
        dstY0: GLint,
        dstX1: GLint,
        dstY1: GLint,
        mask: GLbitfield,
        filter: GLenum
    ) -> (),
    fn glStencilMask(mask: GLuint) -> (),
    fn glStencilMaskSeparate(face: GLenum, mask: GLuint) -> (),
    fn glAttachShader(program: GLuint, shader: GLuint) -> (),
    fn glDetachShader(program: GLuint, shader: GLuint) -> (),
    fn glGetError() -> GLenum,
    fn glClearColor(red: GLclampf, green: GLclampf, blue: GLclampf, alpha: GLclampf) -> (),
    fn glBlendColor(red: GLclampf, green: GLclampf, blue: GLclampf, alpha: GLclampf) -> (),
    fn glTexParameterf(target: GLenum, pname: GLenum, param: GLfloat) -> (),
    fn glTexParameterfv(target: GLenum, pname: GLenum, params: *const GLfloat) -> (),
    fn glGetShaderInfoLog(
        shader: GLuint,
        bufSize: GLsizei,
        length: *mut GLsizei,
        infoLog: *mut GLchar
    ) -> (),
    fn glDepthFunc(func: GLenum) -> (),
    fn glStencilOp(fail: GLenum, zfail: GLenum, zpass: GLenum) -> (),
    fn glStencilFunc(func: GLenum, ref_: GLint, mask: GLuint) -> (),
    fn glEnableVertexAttribArray(index: GLuint) -> (),
    fn glBlendFunc(sfactor: GLenum, dfactor: GLenum) -> (),
    fn glReadBuffer(mode: GLenum) -> (),
    fn glClear(mask: GLbitfield) -> (),
    fn glTexImage2D(
        target: GLenum,
        level: GLint,
        internalFormat: GLint,
        width: GLsizei,
        height: GLsizei,
        border: GLint,
        format: GLenum,
        type_: GLenum,
        pixels: *const GLvoid
    ) -> (),
    fn glGenVertexArrays(n: GLsizei, arrays: *mut GLuint) -> (),
    fn glFrontFace(mode: GLenum) -> (),
    fn glCullFace(mode: GLenum) -> (),
    fn glGenTextures(n: GLsizei, textures: *mut GLuint) -> (),
    fn glReadPixels(
        x: GLint,
        y: GLint,
        width: GLsizei,
        height: GLsizei,
        format: GLenum,
        type_: GLenum,
        pixels: *mut GLvoid
    ) -> (),
    fn glBeginQuery(target: GLenum, id: GLuint) -> (),
    fn glDeleteQueries(n: GLsizei, ids: *const GLuint) -> (),
    fn glEndQuery(target: GLenum) -> (),
    fn glGenQueries(n: GLsizei, ids: *mut GLuint) -> (),
    fn glGetQueryObjectiv(id: GLuint, pname: GLenum, params: *mut GLint) -> (),
    fn glGetQueryObjectui64v(id: GLuint, pname: GLenum, params: *mut GLuint64) -> (),
    fn glFlush() -> (),
    fn glFinish() -> (),
    fn glPolygonMode(face: GLenum, mode: GLenum) -> (),

    //Additional Line
    fn glLineWidth(width: GLfloat) -> (),
    fn glHint(hint: GLenum,value: GLenum) -> (),

    //Additional OpenGL 4.3 for Compute Shaders and SSBO
    fn glDispatchCompute(num_groups_x: GLuint, num_groups_y: GLuint, num_groups_z: GLuint) -> (),
    fn glMemoryBarrier( barriers: GLbitfield) -> (),
    fn glBindBufferBase(target: GLenum, index: GLuint, buffer: GLuint) -> (),
    fn glBindBufferRange(target: GLenum, index: GLuint, buffer: GLuint, offset: GLintptr, size: GLsizeiptr) -> (),
    fn glMapBufferRange(target: GLenum, offset: GLintptr, length: GLsizeiptr, access: GLbitfield) -> (),
    fn glUnmapBuffer(target: GLenum) -> (),
    fn glGetIntegeri_v(target: GLenum, index: GLuint, data: *mut GLint) -> ()
);

// note that glGetString only works after first glSwapBuffer,
// not just after context creation
pub unsafe fn is_gl2() -> bool {
    let version_string = glGetString(super::gl::GL_VERSION);
    let version_string = std::ffi::CStr::from_ptr(version_string as _)
        .to_str()
        .unwrap();

    version_string.is_empty()
        || version_string.starts_with("2")
        || version_string.starts_with("OpenGL ES 2")
}

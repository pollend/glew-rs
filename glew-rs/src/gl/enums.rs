use std::fmt;
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VariantCapEXT(pub(crate) u64);
impl VariantCapEXT {
    pub const GL_VARIANT_ARRAY_EXT: Self = Self(0x87e8);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WeightPointerTypeARB(pub(crate) u64);
impl WeightPointerTypeARB {
    pub const GL_BYTE: Self = Self(0x1400);
    pub const GL_INT: Self = Self(0x1404);
    pub const GL_SHORT: Self = Self(0x1402);
    pub const GL_DOUBLE: Self = Self(0x140a);
    pub const GL_UNSIGNED_BYTE: Self = Self(0x1401);
    pub const GL_UNSIGNED_INT: Self = Self(0x1405);
    pub const GL_UNSIGNED_SHORT: Self = Self(0x1403);
    pub const GL_FLOAT: Self = Self(0x1406);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SamplePatternSGIS(pub(crate) u64);
impl SamplePatternSGIS {
    pub const GL_4PASS_1_SGIS: Self = Self(0x80a5);
    pub const GL_1PASS_SGIS: Self = Self(0x80a1);
    pub const GL_2PASS_1_EXT: Self = Self(0x80a3);
    pub const GL_2PASS_1_SGIS: Self = Self(0x80a3);
    pub const GL_4PASS_0_EXT: Self = Self(0x80a4);
    pub const GL_4PASS_1_EXT: Self = Self(0x80a5);
    pub const GL_4PASS_2_EXT: Self = Self(0x80a6);
    pub const GL_2PASS_0_SGIS: Self = Self(0x80a2);
    pub const GL_4PASS_3_SGIS: Self = Self(0x80a7);
    pub const GL_1PASS_EXT: Self = Self(0x80a1);
    pub const GL_2PASS_0_EXT: Self = Self(0x80a2);
    pub const GL_4PASS_0_SGIS: Self = Self(0x80a4);
    pub const GL_4PASS_2_SGIS: Self = Self(0x80a6);
    pub const GL_4PASS_3_EXT: Self = Self(0x80a7);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RenderingMode(pub(crate) u64);
impl RenderingMode {
    pub const GL_RENDER: Self = Self(0x1c00);
    pub const GL_SELECT: Self = Self(0x1c02);
    pub const GL_FEEDBACK: Self = Self(0x1c01);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AlphaFunction(pub(crate) u64);
impl AlphaFunction {
    pub const GL_GREATER: Self = Self(0x0204);
    pub const GL_NOTEQUAL: Self = Self(0x0205);
    pub const GL_EQUAL: Self = Self(0x0202);
    pub const GL_GEQUAL: Self = Self(0x0206);
    pub const GL_NEVER: Self = Self(0x0200);
    pub const GL_LEQUAL: Self = Self(0x0203);
    pub const GL_LESS: Self = Self(0x0201);
    pub const GL_ALWAYS: Self = Self(0x0207);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FogPName(pub(crate) u64);
impl FogPName {
    pub const GL_FOG_COORD_SRC: Self = Self(0x8450);
    pub const GL_FOG_MODE: Self = Self(0x0b65);
    pub const GL_FOG_INDEX: Self = Self(0x0b61);
    pub const GL_FOG_DENSITY: Self = Self(0x0b62);
    pub const GL_FOG_START: Self = Self(0x0b63);
    pub const GL_FOG_END: Self = Self(0x0b64);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct OcclusionQueryParameterNameNV(pub(crate) u64);
impl OcclusionQueryParameterNameNV {
    pub const GL_PIXEL_COUNT_NV: Self = Self(0x8866);
    pub const GL_PIXEL_COUNT_AVAILABLE_NV: Self = Self(0x8867);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VertexProvokingMode(pub(crate) u64);
impl VertexProvokingMode {
    pub const GL_FIRST_VERTEX_CONVENTION: Self = Self(0x8e4d);
    pub const GL_LAST_VERTEX_CONVENTION: Self = Self(0x8e4e);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SyncCondition(pub(crate) u64);
impl SyncCondition {
    pub const GL_SYNC_GPU_COMMANDS_COMPLETE: Self = Self(0x9117);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ShadingRateQCOM(pub(crate) u64);
impl ShadingRateQCOM {
    pub const GL_SHADING_RATE_4X1_PIXELS_QCOM: Self = Self(0x96ab);
    pub const GL_SHADING_RATE_4X2_PIXELS_QCOM: Self = Self(0x96ac);
    pub const GL_SHADING_RATE_1X4_PIXELS_QCOM: Self = Self(0x96aa);
    pub const GL_SHADING_RATE_2X2_PIXELS_QCOM: Self = Self(0x96a9);
    pub const GL_SHADING_RATE_4X4_PIXELS_QCOM: Self = Self(0x96ae);
    pub const GL_SHADING_RATE_2X1_PIXELS_QCOM: Self = Self(0x96a8);
    pub const GL_SHADING_RATE_1X2_PIXELS_QCOM: Self = Self(0x96a7);
    pub const GL_SHADING_RATE_1X1_PIXELS_QCOM: Self = Self(0x96a6);
    pub const GL_SHADING_RATE_2X4_PIXELS_QCOM: Self = Self(0x96ad);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TextureEnvMode(pub(crate) u64);
impl TextureEnvMode {
    pub const GL_ADD: Self = Self(0x0104);
    pub const GL_BLEND: Self = Self(0x0be2);
    pub const GL_DECAL: Self = Self(0x2101);
    pub const GL_TEXTURE_ENV_BIAS_SGIX: Self = Self(0x80be);
    pub const GL_MODULATE: Self = Self(0x2100);
    pub const GL_REPLACE_EXT: Self = Self(0x8062);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EnableCap(pub(crate) u64);
impl EnableCap {
    pub const GL_INDEX_LOGIC_OP: Self = Self(0x0bf1);
    pub const GL_DEPTH_CLAMP: Self = Self(0x864f);
    pub const GL_RASTERIZER_DISCARD: Self = Self(0x8c89);
    pub const GL_CLIP_PLANE5: Self = Self(0x3005);
    pub const GL_STENCIL_TEST: Self = Self(0x0b90);
    pub const GL_FOG_OFFSET_SGIX: Self = Self(0x8198);
    pub const GL_INDEX_ARRAY: Self = Self(0x8077);
    pub const GL_LIGHT6: Self = Self(0x4006);
    pub const GL_ALPHA_TEST: Self = Self(0x0bc0);
    pub const GL_CONVOLUTION_1D_EXT: Self = Self(0x8010);
    pub const GL_MAP2_VERTEX_4: Self = Self(0x0db8);
    pub const GL_CLIP_DISTANCE0: Self = Self(0x3000);
    pub const GL_MAP1_INDEX: Self = Self(0x0d91);
    pub const GL_MAP1_VERTEX_4: Self = Self(0x0d98);
    pub const GL_NORMAL_ARRAY: Self = Self(0x8075);
    pub const GL_LIGHT5: Self = Self(0x4005);
    pub const GL_FRAGMENT_LIGHT4_SGIX: Self = Self(0x8410);
    pub const GL_MAP1_VERTEX_3: Self = Self(0x0d97);
    pub const GL_SHADING_RATE_PRESERVE_ASPECT_RATIO_QCOM: Self = Self(0x96a5);
    pub const GL_COLOR_ARRAY: Self = Self(0x8076);
    pub const GL_LINE_STIPPLE: Self = Self(0x0b24);
    pub const GL_RESCALE_NORMAL_EXT: Self = Self(0x803a);
    pub const GL_ASYNC_HISTOGRAM_SGIX: Self = Self(0x832c);
    pub const GL_TEXTURE_RECTANGLE: Self = Self(0x84f5);
    pub const GL_CLIP_PLANE1: Self = Self(0x3001);
    pub const GL_SAMPLE_ALPHA_TO_ONE_SGIS: Self = Self(0x809f);
    pub const GL_SAMPLE_SHADING: Self = Self(0x8c36);
    pub const GL_MULTISAMPLE: Self = Self(0x809d);
    pub const GL_DITHER: Self = Self(0x0bd0);
    pub const GL_FRAGMENT_LIGHT3_SGIX: Self = Self(0x840f);
    pub const GL_MAP2_TEXTURE_COORD_1: Self = Self(0x0db3);
    pub const GL_TEXTURE_RECTANGLE_NV: Self = Self(0x84f5);
    pub const GL_FRAGMENT_LIGHTING_SGIX: Self = Self(0x8400);
    pub const GL_LIGHT1: Self = Self(0x4001);
    pub const GL_TEXTURE_CUBE_MAP: Self = Self(0x8513);
    pub const GL_SHARED_TEXTURE_PALETTE_EXT: Self = Self(0x81fb);
    pub const GL_FRAMEBUFFER_SRGB: Self = Self(0x8db9);
    pub const GL_TEXTURE_1D: Self = Self(0x0de0);
    pub const GL_FRAMEZOOM_SGIX: Self = Self(0x818b);
    pub const GL_FRAGMENT_LIGHT5_SGIX: Self = Self(0x8411);
    pub const GL_MAP1_TEXTURE_COORD_1: Self = Self(0x0d93);
    pub const GL_SPRITE_SGIX: Self = Self(0x8148);
    pub const GL_MAP2_TEXTURE_COORD_3: Self = Self(0x0db5);
    pub const GL_FRAGMENT_LIGHT0_SGIX: Self = Self(0x840c);
    pub const GL_FRAGMENT_LIGHT1_SGIX: Self = Self(0x840d);
    pub const GL_POST_CONVOLUTION_COLOR_TABLE_SGI: Self = Self(0x80d1);
    pub const GL_TEXTURE_CUBE_MAP_SEAMLESS: Self = Self(0x884f);
    pub const GL_LIGHT0: Self = Self(0x4000);
    pub const GL_DEPTH_TEST: Self = Self(0x0b71);
    pub const GL_TEXTURE_GEN_S: Self = Self(0x0c60);
    pub const GL_MAP2_VERTEX_3: Self = Self(0x0db7);
    pub const GL_TEXTURE_3D_EXT: Self = Self(0x806f);
    pub const GL_TEXTURE_CUBE_MAP_EXT: Self = Self(0x8513);
    pub const GL_TEXTURE_2D: Self = Self(0x0de1);
    pub const GL_SAMPLE_ALPHA_TO_ONE: Self = Self(0x809f);
    pub const GL_MAP2_INDEX: Self = Self(0x0db1);
    pub const GL_TEXTURE_COLOR_TABLE_SGI: Self = Self(0x80bc);
    pub const GL_PRIMITIVE_RESTART_FIXED_INDEX: Self = Self(0x8d69);
    pub const GL_DEBUG_OUTPUT_SYNCHRONOUS: Self = Self(0x8242);
    pub const GL_SAMPLE_ALPHA_TO_COVERAGE: Self = Self(0x809e);
    pub const GL_TEXTURE_COORD_ARRAY: Self = Self(0x8078);
    pub const GL_POLYGON_OFFSET_POINT: Self = Self(0x2a01);
    pub const GL_LIGHT7: Self = Self(0x4007);
    pub const GL_MAP1_NORMAL: Self = Self(0x0d92);
    pub const GL_EDGE_FLAG_ARRAY: Self = Self(0x8079);
    pub const GL_CLIP_DISTANCE2: Self = Self(0x3002);
    pub const GL_FRAGMENT_LIGHT2_SGIX: Self = Self(0x840e);
    pub const GL_TEXTURE_RECTANGLE_ARB: Self = Self(0x84f5);
    pub const GL_CLIP_DISTANCE3: Self = Self(0x3003);
    pub const GL_CLIP_DISTANCE6: Self = Self(0x3006);
    pub const GL_FRAGMENT_LIGHT7_SGIX: Self = Self(0x8413);
    pub const GL_CLIP_DISTANCE5: Self = Self(0x3005);
    pub const GL_FETCH_PER_SAMPLE_ARM: Self = Self(0x8f65);
    pub const GL_PRIMITIVE_RESTART: Self = Self(0x8f9d);
    pub const GL_CALLIGRAPHIC_FRAGMENT_SGIX: Self = Self(0x8183);
    pub const GL_POST_COLOR_MATRIX_COLOR_TABLE: Self = Self(0x80d2);
    pub const GL_SAMPLE_MASK: Self = Self(0x8e51);
    pub const GL_VERTEX_ARRAY: Self = Self(0x8074);
    pub const GL_CLIP_PLANE3: Self = Self(0x3003);
    pub const GL_LIGHT4: Self = Self(0x4004);
    pub const GL_CLIP_PLANE0: Self = Self(0x3000);
    pub const GL_NORMALIZE: Self = Self(0x0ba1);
    pub const GL_MAP2_NORMAL: Self = Self(0x0db2);
    pub const GL_SAMPLE_ALPHA_TO_MASK_SGIS: Self = Self(0x809e);
    pub const GL_TEXTURE_GEN_Q: Self = Self(0x0c63);
    pub const GL_MAP1_COLOR_4: Self = Self(0x0d90);
    pub const GL_LINE_SMOOTH: Self = Self(0x0b20);
    pub const GL_POLYGON_OFFSET_LINE: Self = Self(0x2a02);
    pub const GL_POLYGON_STIPPLE: Self = Self(0x0b42);
    pub const GL_TEXTURE_4D_SGIS: Self = Self(0x8134);
    pub const GL_CLIP_PLANE4: Self = Self(0x3004);
    pub const GL_PIXEL_TEXTURE_SGIS: Self = Self(0x8353);
    pub const GL_POLYGON_SMOOTH: Self = Self(0x0b41);
    pub const GL_BLEND: Self = Self(0x0be2);
    pub const GL_PROGRAM_POINT_SIZE: Self = Self(0x8642);
    pub const GL_MAP2_TEXTURE_COORD_2: Self = Self(0x0db4);
    pub const GL_SEPARABLE_2D_EXT: Self = Self(0x8012);
    pub const GL_POST_CONVOLUTION_COLOR_TABLE: Self = Self(0x80d1);
    pub const GL_PIXEL_TEX_GEN_SGIX: Self = Self(0x8139);
    pub const GL_SAMPLE_COVERAGE: Self = Self(0x80a0);
    pub const GL_ASYNC_TEX_IMAGE_SGIX: Self = Self(0x835c);
    pub const GL_AUTO_NORMAL: Self = Self(0x0d80);
    pub const GL_ASYNC_READ_PIXELS_SGIX: Self = Self(0x835e);
    pub const GL_SHADING_RATE_IMAGE_PER_PRIMITIVE_NV: Self = Self(0x95b1);
    pub const GL_LIGHTING: Self = Self(0x0b50);
    pub const GL_SAMPLE_MASK_SGIS: Self = Self(0x80a0);
    pub const GL_CULL_FACE: Self = Self(0x0b44);
    pub const GL_MULTISAMPLE_SGIS: Self = Self(0x809d);
    pub const GL_POST_COLOR_MATRIX_COLOR_TABLE_SGI: Self = Self(0x80d2);
    pub const GL_COLOR_LOGIC_OP: Self = Self(0x0bf2);
    pub const GL_CLIP_DISTANCE1: Self = Self(0x3001);
    pub const GL_POINT_SMOOTH: Self = Self(0x0b10);
    pub const GL_LIGHT2: Self = Self(0x4002);
    pub const GL_CONVOLUTION_2D_EXT: Self = Self(0x8011);
    pub const GL_COLOR_TABLE_SGI: Self = Self(0x80d0);
    pub const GL_COLOR_TABLE: Self = Self(0x80d0);
    pub const GL_LIGHT3: Self = Self(0x4003);
    pub const GL_TEXTURE_CUBE_MAP_ARB: Self = Self(0x8513);
    pub const GL_REFERENCE_PLANE_SGIX: Self = Self(0x817d);
    pub const GL_TEXTURE_CUBE_MAP_OES: Self = Self(0x8513);
    pub const GL_COLOR_MATERIAL: Self = Self(0x0b57);
    pub const GL_FOG: Self = Self(0x0b60);
    pub const GL_MAP2_TEXTURE_COORD_4: Self = Self(0x0db6);
    pub const GL_FRAGMENT_COLOR_MATERIAL_SGIX: Self = Self(0x8401);
    pub const GL_HISTOGRAM_EXT: Self = Self(0x8024);
    pub const GL_MAP2_COLOR_4: Self = Self(0x0db0);
    pub const GL_CLIP_DISTANCE4: Self = Self(0x3004);
    pub const GL_SCISSOR_TEST: Self = Self(0x0c11);
    pub const GL_MINMAX_EXT: Self = Self(0x802e);
    pub const GL_ASYNC_DRAW_PIXELS_SGIX: Self = Self(0x835d);
    pub const GL_IR_INSTRUMENT1_SGIX: Self = Self(0x817f);
    pub const GL_MAP1_TEXTURE_COORD_3: Self = Self(0x0d95);
    pub const GL_CLIP_PLANE2: Self = Self(0x3002);
    pub const GL_FRAGMENT_LIGHT6_SGIX: Self = Self(0x8412);
    pub const GL_DEBUG_OUTPUT: Self = Self(0x92e0);
    pub const GL_MAP1_TEXTURE_COORD_2: Self = Self(0x0d94);
    pub const GL_MAP1_TEXTURE_COORD_4: Self = Self(0x0d96);
    pub const GL_POLYGON_OFFSET_FILL: Self = Self(0x8037);
    pub const GL_INTERLACE_SGIX: Self = Self(0x8094);
    pub const GL_TEXTURE_GEN_T: Self = Self(0x0c61);
    pub const GL_TEXTURE_GEN_R: Self = Self(0x0c62);
    pub const GL_CLIP_DISTANCE7: Self = Self(0x3007);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GlslTypeToken(pub(crate) u64);
impl GlslTypeToken {
    pub const GL_UNSIGNED_INT_SAMPLER_2D_MULTISAMPLE_ARRAY: Self = Self(0x910d);
    pub const GL_INT_VEC2: Self = Self(0x8b53);
    pub const GL_INT_SAMPLER_CUBE: Self = Self(0x8dcc);
    pub const GL_INT_IMAGE_2D_RECT: Self = Self(0x905a);
    pub const GL_UNSIGNED_INT_SAMPLER_2D_ARRAY: Self = Self(0x8dd7);
    pub const GL_UNSIGNED_INT_IMAGE_1D_ARRAY: Self = Self(0x9068);
    pub const GL_UNSIGNED_INT_IMAGE_2D_ARRAY: Self = Self(0x9069);
    pub const GL_SAMPLER_2D_RECT_SHADOW: Self = Self(0x8b64);
    pub const GL_INT_SAMPLER_2D_ARRAY: Self = Self(0x8dcf);
    pub const GL_INT_SAMPLER_2D: Self = Self(0x8dca);
    pub const GL_UNSIGNED_INT_SAMPLER_CUBE: Self = Self(0x8dd4);
    pub const GL_FLOAT_MAT2x4: Self = Self(0x8b66);
    pub const GL_DOUBLE_VEC3: Self = Self(0x8ffd);
    pub const GL_IMAGE_2D_RECT: Self = Self(0x904f);
    pub const GL_DOUBLE_MAT4: Self = Self(0x8f48);
    pub const GL_UNSIGNED_INT_IMAGE_2D_MULTISAMPLE: Self = Self(0x906b);
    pub const GL_FLOAT_MAT2x3: Self = Self(0x8b65);
    pub const GL_UNSIGNED_INT_IMAGE_CUBE_MAP_ARRAY: Self = Self(0x906a);
    pub const GL_SAMPLER_2D_MULTISAMPLE_ARRAY: Self = Self(0x910b);
    pub const GL_DOUBLE_MAT2: Self = Self(0x8f46);
    pub const GL_SAMPLER_1D_SHADOW: Self = Self(0x8b61);
    pub const GL_SAMPLER_CUBE: Self = Self(0x8b60);
    pub const GL_INT_IMAGE_CUBE_MAP_ARRAY: Self = Self(0x905f);
    pub const GL_DOUBLE_VEC2: Self = Self(0x8ffc);
    pub const GL_INT_VEC4: Self = Self(0x8b55);
    pub const GL_INT: Self = Self(0x1404);
    pub const GL_INT_IMAGE_CUBE: Self = Self(0x905b);
    pub const GL_SAMPLER_BUFFER: Self = Self(0x8dc2);
    pub const GL_UNSIGNED_INT_VEC4: Self = Self(0x8dc8);
    pub const GL_FLOAT_MAT4: Self = Self(0x8b5c);
    pub const GL_UNSIGNED_INT_SAMPLER_BUFFER: Self = Self(0x8dd8);
    pub const GL_UNSIGNED_INT_SAMPLER_2D: Self = Self(0x8dd2);
    pub const GL_UNSIGNED_INT_SAMPLER_CUBE_MAP_ARRAY: Self = Self(0x900f);
    pub const GL_IMAGE_2D: Self = Self(0x904d);
    pub const GL_SAMPLER_1D_ARRAY_SHADOW: Self = Self(0x8dc3);
    pub const GL_FLOAT_MAT4x2: Self = Self(0x8b69);
    pub const GL_UNSIGNED_INT_IMAGE_1D: Self = Self(0x9062);
    pub const GL_UNSIGNED_INT_ATOMIC_COUNTER: Self = Self(0x92db);
    pub const GL_UNSIGNED_INT_SAMPLER_1D_ARRAY: Self = Self(0x8dd6);
    pub const GL_FLOAT_VEC3: Self = Self(0x8b51);
    pub const GL_SAMPLER_3D: Self = Self(0x8b5f);
    pub const GL_SAMPLER_2D_ARRAY_SHADOW: Self = Self(0x8dc4);
    pub const GL_DOUBLE_MAT3: Self = Self(0x8f47);
    pub const GL_DOUBLE_VEC4: Self = Self(0x8ffe);
    pub const GL_INT_IMAGE_1D: Self = Self(0x9057);
    pub const GL_INT_IMAGE_BUFFER: Self = Self(0x905c);
    pub const GL_UNSIGNED_INT_IMAGE_BUFFER: Self = Self(0x9067);
    pub const GL_UNSIGNED_INT_VEC3: Self = Self(0x8dc7);
    pub const GL_IMAGE_2D_MULTISAMPLE: Self = Self(0x9055);
    pub const GL_INT_IMAGE_1D_ARRAY: Self = Self(0x905d);
    pub const GL_UNSIGNED_INT_IMAGE_2D: Self = Self(0x9063);
    pub const GL_SAMPLER_2D_ARRAY: Self = Self(0x8dc1);
    pub const GL_SAMPLER_1D_ARRAY: Self = Self(0x8dc0);
    pub const GL_DOUBLE: Self = Self(0x140a);
    pub const GL_UNSIGNED_INT_SAMPLER_3D: Self = Self(0x8dd3);
    pub const GL_UNSIGNED_INT_SAMPLER_1D: Self = Self(0x8dd1);
    pub const GL_UNSIGNED_INT: Self = Self(0x1405);
    pub const GL_BOOL_VEC4: Self = Self(0x8b59);
    pub const GL_INT_SAMPLER_CUBE_MAP_ARRAY: Self = Self(0x900e);
    pub const GL_INT_IMAGE_2D: Self = Self(0x9058);
    pub const GL_FLOAT_MAT3x2: Self = Self(0x8b67);
    pub const GL_FLOAT_VEC4: Self = Self(0x8b52);
    pub const GL_SAMPLER_1D: Self = Self(0x8b5d);
    pub const GL_FLOAT_MAT3x4: Self = Self(0x8b68);
    pub const GL_SAMPLER_CUBE_SHADOW: Self = Self(0x8dc5);
    pub const GL_IMAGE_3D: Self = Self(0x904e);
    pub const GL_INT_IMAGE_2D_ARRAY: Self = Self(0x905e);
    pub const GL_UNSIGNED_INT_IMAGE_2D_MULTISAMPLE_ARRAY: Self = Self(0x906c);
    pub const GL_FLOAT_MAT2: Self = Self(0x8b5a);
    pub const GL_INT_SAMPLER_2D_MULTISAMPLE: Self = Self(0x9109);
    pub const GL_SAMPLER_CUBE_MAP_ARRAY_SHADOW: Self = Self(0x900d);
    pub const GL_IMAGE_2D_ARRAY: Self = Self(0x9053);
    pub const GL_INT_SAMPLER_BUFFER: Self = Self(0x8dd0);
    pub const GL_INT_SAMPLER_2D_MULTISAMPLE_ARRAY: Self = Self(0x910c);
    pub const GL_SAMPLER_2D: Self = Self(0x8b5e);
    pub const GL_INT_SAMPLER_3D: Self = Self(0x8dcb);
    pub const GL_INT_SAMPLER_2D_RECT: Self = Self(0x8dcd);
    pub const GL_UNSIGNED_INT_SAMPLER_2D_MULTISAMPLE: Self = Self(0x910a);
    pub const GL_FLOAT_MAT3: Self = Self(0x8b5b);
    pub const GL_IMAGE_2D_MULTISAMPLE_ARRAY: Self = Self(0x9056);
    pub const GL_INT_IMAGE_2D_MULTISAMPLE: Self = Self(0x9060);
    pub const GL_INT_VEC3: Self = Self(0x8b54);
    pub const GL_UNSIGNED_INT_IMAGE_2D_RECT: Self = Self(0x9065);
    pub const GL_UNSIGNED_INT_IMAGE_3D: Self = Self(0x9064);
    pub const GL_IMAGE_CUBE_MAP_ARRAY: Self = Self(0x9054);
    pub const GL_INT_SAMPLER_1D: Self = Self(0x8dc9);
    pub const GL_INT_SAMPLER_1D_ARRAY: Self = Self(0x8dce);
    pub const GL_INT_IMAGE_3D: Self = Self(0x9059);
    pub const GL_UNSIGNED_INT_VEC2: Self = Self(0x8dc6);
    pub const GL_SAMPLER_2D_SHADOW: Self = Self(0x8b62);
    pub const GL_IMAGE_1D: Self = Self(0x904c);
    pub const GL_BOOL: Self = Self(0x8b56);
    pub const GL_IMAGE_CUBE: Self = Self(0x9050);
    pub const GL_INT_IMAGE_2D_MULTISAMPLE_ARRAY: Self = Self(0x9061);
    pub const GL_UNSIGNED_INT_IMAGE_CUBE: Self = Self(0x9066);
    pub const GL_FLOAT: Self = Self(0x1406);
    pub const GL_FLOAT_MAT4x3: Self = Self(0x8b6a);
    pub const GL_UNSIGNED_INT_SAMPLER_2D_RECT: Self = Self(0x8dd5);
    pub const GL_IMAGE_1D_ARRAY: Self = Self(0x9052);
    pub const GL_BOOL_VEC2: Self = Self(0x8b57);
    pub const GL_SAMPLER_2D_RECT: Self = Self(0x8b63);
    pub const GL_BOOL_VEC3: Self = Self(0x8b58);
    pub const GL_FLOAT_VEC2: Self = Self(0x8b50);
    pub const GL_SAMPLER_CUBE_MAP_ARRAY: Self = Self(0x900c);
    pub const GL_IMAGE_BUFFER: Self = Self(0x9051);
    pub const GL_SAMPLER_2D_MULTISAMPLE: Self = Self(0x9108);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LogicOp(pub(crate) u64);
impl LogicOp {
    pub const GL_NOOP: Self = Self(0x1505);
    pub const GL_AND_REVERSE: Self = Self(0x1502);
    pub const GL_NAND: Self = Self(0x150e);
    pub const GL_COPY_INVERTED: Self = Self(0x150c);
    pub const GL_AND_INVERTED: Self = Self(0x1504);
    pub const GL_XOR: Self = Self(0x1506);
    pub const GL_NOR: Self = Self(0x1508);
    pub const GL_INVERT: Self = Self(0x150a);
    pub const GL_OR_INVERTED: Self = Self(0x150d);
    pub const GL_EQUIV: Self = Self(0x1509);
    pub const GL_SET: Self = Self(0x150f);
    pub const GL_CLEAR: Self = Self(0x1500);
    pub const GL_COPY: Self = Self(0x1503);
    pub const GL_OR: Self = Self(0x1507);
    pub const GL_OR_REVERSE: Self = Self(0x150b);
    pub const GL_AND: Self = Self(0x1501);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VertexShaderOpEXT(pub(crate) u64);
impl VertexShaderOpEXT {
    pub const GL_OP_FLOOR_EXT: Self = Self(0x878f);
    pub const GL_OP_RECIP_EXT: Self = Self(0x8794);
    pub const GL_OP_MOV_EXT: Self = Self(0x8799);
    pub const GL_OP_DOT4_EXT: Self = Self(0x8785);
    pub const GL_OP_NEGATE_EXT: Self = Self(0x8783);
    pub const GL_OP_POWER_EXT: Self = Self(0x8793);
    pub const GL_OP_MAX_EXT: Self = Self(0x878a);
    pub const GL_OP_MUL_EXT: Self = Self(0x8786);
    pub const GL_OP_SET_GE_EXT: Self = Self(0x878c);
    pub const GL_OP_LOG_BASE_2_EXT: Self = Self(0x8792);
    pub const GL_OP_MADD_EXT: Self = Self(0x8788);
    pub const GL_OP_ROUND_EXT: Self = Self(0x8790);
    pub const GL_OP_EXP_BASE_2_EXT: Self = Self(0x8791);
    pub const GL_OP_ADD_EXT: Self = Self(0x8787);
    pub const GL_OP_INDEX_EXT: Self = Self(0x8782);
    pub const GL_OP_FRAC_EXT: Self = Self(0x8789);
    pub const GL_OP_CLAMP_EXT: Self = Self(0x878e);
    pub const GL_OP_MIN_EXT: Self = Self(0x878b);
    pub const GL_OP_DOT3_EXT: Self = Self(0x8784);
    pub const GL_OP_RECIP_SQRT_EXT: Self = Self(0x8795);
    pub const GL_OP_MULTIPLY_MATRIX_EXT: Self = Self(0x8798);
    pub const GL_OP_CROSS_PRODUCT_EXT: Self = Self(0x8797);
    pub const GL_OP_SET_LT_EXT: Self = Self(0x878d);
    pub const GL_OP_SUB_EXT: Self = Self(0x8796);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ListParameterName(pub(crate) u64);
impl ListParameterName {
    pub const GL_LIST_PRIORITY_SGIX: Self = Self(0x8182);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ArrayObjectPNameATI(pub(crate) u64);
impl ArrayObjectPNameATI {
    pub const GL_OBJECT_BUFFER_SIZE_ATI: Self = Self(0x8764);
    pub const GL_OBJECT_BUFFER_USAGE_ATI: Self = Self(0x8765);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MapTarget(pub(crate) u64);
impl MapTarget {
    pub const GL_GEOMETRY_DEFORMATION_SGIX: Self = Self(0x8194);
    pub const GL_MAP2_VERTEX_3: Self = Self(0x0db7);
    pub const GL_MAP1_VERTEX_4: Self = Self(0x0d98);
    pub const GL_MAP1_TEXTURE_COORD_1: Self = Self(0x0d93);
    pub const GL_MAP2_INDEX: Self = Self(0x0db1);
    pub const GL_MAP2_VERTEX_4: Self = Self(0x0db8);
    pub const GL_TEXTURE_DEFORMATION_SGIX: Self = Self(0x8195);
    pub const GL_MAP1_COLOR_4: Self = Self(0x0d90);
    pub const GL_MAP2_NORMAL: Self = Self(0x0db2);
    pub const GL_MAP2_TEXTURE_COORD_3: Self = Self(0x0db5);
    pub const GL_MAP1_NORMAL: Self = Self(0x0d92);
    pub const GL_MAP1_TEXTURE_COORD_2: Self = Self(0x0d94);
    pub const GL_MAP1_TEXTURE_COORD_4: Self = Self(0x0d96);
    pub const GL_MAP1_VERTEX_3: Self = Self(0x0d97);
    pub const GL_MAP1_TEXTURE_COORD_3: Self = Self(0x0d95);
    pub const GL_MAP2_TEXTURE_COORD_1: Self = Self(0x0db3);
    pub const GL_MAP1_INDEX: Self = Self(0x0d91);
    pub const GL_MAP2_TEXTURE_COORD_2: Self = Self(0x0db4);
    pub const GL_MAP2_COLOR_4: Self = Self(0x0db0);
    pub const GL_MAP2_TEXTURE_COORD_4: Self = Self(0x0db6);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FogCoordinatePointerType(pub(crate) u64);
impl FogCoordinatePointerType {
    pub const GL_FLOAT: Self = Self(0x1406);
    pub const GL_DOUBLE: Self = Self(0x140a);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PointParameterNameARB(pub(crate) u64);
impl PointParameterNameARB {
    pub const GL_POINT_SIZE_MIN_EXT: Self = Self(0x8126);
    pub const GL_POINT_FADE_THRESHOLD_SIZE: Self = Self(0x8128);
    pub const GL_POINT_SIZE_MAX_EXT: Self = Self(0x8127);
    pub const GL_POINT_FADE_THRESHOLD_SIZE_EXT: Self = Self(0x8128);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PixelTransformPNameEXT(pub(crate) u64);
impl PixelTransformPNameEXT {
    pub const GL_PIXEL_MIN_FILTER_EXT: Self = Self(0x8332);
    pub const GL_PIXEL_MAG_FILTER_EXT: Self = Self(0x8331);
    pub const GL_PIXEL_CUBIC_WEIGHT_EXT: Self = Self(0x8333);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BufferUsageARB(pub(crate) u64);
impl BufferUsageARB {
    pub const GL_STREAM_DRAW: Self = Self(0x88e0);
    pub const GL_STATIC_DRAW: Self = Self(0x88e4);
    pub const GL_STATIC_READ: Self = Self(0x88e5);
    pub const GL_STREAM_COPY: Self = Self(0x88e2);
    pub const GL_DYNAMIC_DRAW: Self = Self(0x88e8);
    pub const GL_DYNAMIC_COPY: Self = Self(0x88ea);
    pub const GL_STATIC_COPY: Self = Self(0x88e6);
    pub const GL_STREAM_READ: Self = Self(0x88e1);
    pub const GL_DYNAMIC_READ: Self = Self(0x88e9);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TransformFeedbackBufferMode(pub(crate) u64);
impl TransformFeedbackBufferMode {
    pub const GL_SEPARATE_ATTRIBS: Self = Self(0x8c8d);
    pub const GL_INTERLEAVED_ATTRIBS: Self = Self(0x8c8c);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TangentPointerTypeEXT(pub(crate) u64);
impl TangentPointerTypeEXT {
    pub const GL_BYTE: Self = Self(0x1400);
    pub const GL_FLOAT: Self = Self(0x1406);
    pub const GL_DOUBLE: Self = Self(0x140a);
    pub const GL_INT: Self = Self(0x1404);
    pub const GL_DOUBLE_EXT: Self = Self(0x140a);
    pub const GL_SHORT: Self = Self(0x1402);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FenceParameterNameNV(pub(crate) u64);
impl FenceParameterNameNV {
    pub const GL_FENCE_CONDITION_NV: Self = Self(0x84f4);
    pub const GL_FENCE_STATUS_NV: Self = Self(0x84f3);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CommandOpcodesNV(pub(crate) u64);
impl CommandOpcodesNV {
    pub const GL_LINE_WIDTH_COMMAND_NV: Self = Self(0x000d);
    pub const GL_DRAW_ELEMENTS_INSTANCED_COMMAND_NV: Self = Self(0x0006);
    pub const GL_NOP_COMMAND_NV: Self = Self(0x0001);
    pub const GL_UNIFORM_ADDRESS_COMMAND_NV: Self = Self(0x000a);
    pub const GL_TERMINATE_SEQUENCE_COMMAND_NV: Self = Self(0x0000);
    pub const GL_DRAW_ARRAYS_INSTANCED_COMMAND_NV: Self = Self(0x0007);
    pub const GL_POLYGON_OFFSET_COMMAND_NV: Self = Self(0x000e);
    pub const GL_VIEWPORT_COMMAND_NV: Self = Self(0x0010);
    pub const GL_SCISSOR_COMMAND_NV: Self = Self(0x0011);
    pub const GL_DRAW_ARRAYS_STRIP_COMMAND_NV: Self = Self(0x0005);
    pub const GL_DRAW_ARRAYS_COMMAND_NV: Self = Self(0x0003);
    pub const GL_FRONT_FACE_COMMAND_NV: Self = Self(0x0012);
    pub const GL_BLEND_COLOR_COMMAND_NV: Self = Self(0x000b);
    pub const GL_ALPHA_REF_COMMAND_NV: Self = Self(0x000f);
    pub const GL_ATTRIBUTE_ADDRESS_COMMAND_NV: Self = Self(0x0009);
    pub const GL_DRAW_ELEMENTS_COMMAND_NV: Self = Self(0x0002);
    pub const GL_DRAW_ELEMENTS_STRIP_COMMAND_NV: Self = Self(0x0004);
    pub const GL_STENCIL_REF_COMMAND_NV: Self = Self(0x000c);
    pub const GL_ELEMENT_ADDRESS_COMMAND_NV: Self = Self(0x0008);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ColorMaterialParameter(pub(crate) u64);
impl ColorMaterialParameter {
    pub const GL_EMISSION: Self = Self(0x1600);
    pub const GL_AMBIENT_AND_DIFFUSE: Self = Self(0x1602);
    pub const GL_AMBIENT: Self = Self(0x1200);
    pub const GL_DIFFUSE: Self = Self(0x1201);
    pub const GL_SPECULAR: Self = Self(0x1202);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TextureNormalModeEXT(pub(crate) u64);
impl TextureNormalModeEXT {
    pub const GL_PERTURB_EXT: Self = Self(0x85ae);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TextureUnit(pub(crate) u64);
impl TextureUnit {
    pub const GL_TEXTURE14: Self = Self(0x84ce);
    pub const GL_TEXTURE21: Self = Self(0x84d5);
    pub const GL_TEXTURE30: Self = Self(0x84de);
    pub const GL_TEXTURE27: Self = Self(0x84db);
    pub const GL_TEXTURE31: Self = Self(0x84df);
    pub const GL_TEXTURE19: Self = Self(0x84d3);
    pub const GL_TEXTURE12: Self = Self(0x84cc);
    pub const GL_TEXTURE1: Self = Self(0x84c1);
    pub const GL_TEXTURE0: Self = Self(0x84c0);
    pub const GL_TEXTURE25: Self = Self(0x84d9);
    pub const GL_TEXTURE15: Self = Self(0x84cf);
    pub const GL_TEXTURE24: Self = Self(0x84d8);
    pub const GL_TEXTURE6: Self = Self(0x84c6);
    pub const GL_TEXTURE23: Self = Self(0x84d7);
    pub const GL_TEXTURE11: Self = Self(0x84cb);
    pub const GL_TEXTURE9: Self = Self(0x84c9);
    pub const GL_TEXTURE2: Self = Self(0x84c2);
    pub const GL_TEXTURE3: Self = Self(0x84c3);
    pub const GL_TEXTURE8: Self = Self(0x84c8);
    pub const GL_TEXTURE13: Self = Self(0x84cd);
    pub const GL_TEXTURE4: Self = Self(0x84c4);
    pub const GL_TEXTURE20: Self = Self(0x84d4);
    pub const GL_TEXTURE26: Self = Self(0x84da);
    pub const GL_TEXTURE22: Self = Self(0x84d6);
    pub const GL_TEXTURE28: Self = Self(0x84dc);
    pub const GL_TEXTURE16: Self = Self(0x84d0);
    pub const GL_TEXTURE29: Self = Self(0x84dd);
    pub const GL_TEXTURE18: Self = Self(0x84d2);
    pub const GL_TEXTURE10: Self = Self(0x84ca);
    pub const GL_TEXTURE5: Self = Self(0x84c5);
    pub const GL_TEXTURE7: Self = Self(0x84c7);
    pub const GL_TEXTURE17: Self = Self(0x84d1);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PixelMap(pub(crate) u64);
impl PixelMap {
    pub const GL_PIXEL_MAP_A_TO_A: Self = Self(0x0c79);
    pub const GL_PIXEL_MAP_I_TO_A: Self = Self(0x0c75);
    pub const GL_PIXEL_MAP_R_TO_R: Self = Self(0x0c76);
    pub const GL_PIXEL_MAP_B_TO_B: Self = Self(0x0c78);
    pub const GL_PIXEL_MAP_I_TO_I: Self = Self(0x0c70);
    pub const GL_PIXEL_MAP_I_TO_G: Self = Self(0x0c73);
    pub const GL_PIXEL_MAP_S_TO_S: Self = Self(0x0c71);
    pub const GL_PIXEL_MAP_G_TO_G: Self = Self(0x0c77);
    pub const GL_PIXEL_MAP_I_TO_R: Self = Self(0x0c72);
    pub const GL_PIXEL_MAP_I_TO_B: Self = Self(0x0c74);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TexCoordPointerType(pub(crate) u64);
impl TexCoordPointerType {
    pub const GL_SHORT: Self = Self(0x1402);
    pub const GL_FLOAT: Self = Self(0x1406);
    pub const GL_INT: Self = Self(0x1404);
    pub const GL_DOUBLE: Self = Self(0x140a);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Boolean(pub(crate) u64);
impl Boolean {
    pub const GL_TRUE: Self = Self(1);
    pub const GL_FALSE: Self = Self(0);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CullFaceMode(pub(crate) u64);
impl CullFaceMode {
    pub const GL_FRONT_AND_BACK: Self = Self(0x0408);
    pub const GL_BACK: Self = Self(0x0405);
    pub const GL_FRONT: Self = Self(0x0404);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LightEnvParameterSGIX(pub(crate) u64);
impl LightEnvParameterSGIX {
    pub const GL_LIGHT_ENV_MODE_SGIX: Self = Self(0x8407);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PixelTexGenMode(pub(crate) u64);
impl PixelTexGenMode {
    pub const GL_PIXEL_TEX_GEN_ALPHA_NO_REPLACE_SGIX: Self = Self(0x8188);
    pub const GL_RGB: Self = Self(0x1907);
    pub const GL_RGBA: Self = Self(0x1908);
    pub const GL_PIXEL_TEX_GEN_ALPHA_LS_SGIX: Self = Self(0x8189);
    pub const GL_PIXEL_TEX_GEN_ALPHA_MS_SGIX: Self = Self(0x818a);
    pub const GL_LUMINANCE: Self = Self(0x1909);
    pub const GL_LUMINANCE_ALPHA: Self = Self(0x190a);
    pub const GL_PIXEL_TEX_GEN_ALPHA_REPLACE_SGIX: Self = Self(0x8187);
    pub const GL_NONE: Self = Self(0);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StencilFunction(pub(crate) u64);
impl StencilFunction {
    pub const GL_ALWAYS: Self = Self(0x0207);
    pub const GL_EQUAL: Self = Self(0x0202);
    pub const GL_LESS: Self = Self(0x0201);
    pub const GL_LEQUAL: Self = Self(0x0203);
    pub const GL_NOTEQUAL: Self = Self(0x0205);
    pub const GL_GEQUAL: Self = Self(0x0206);
    pub const GL_NEVER: Self = Self(0x0200);
    pub const GL_GREATER: Self = Self(0x0204);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BinormalPointerTypeEXT(pub(crate) u64);
impl BinormalPointerTypeEXT {
    pub const GL_DOUBLE: Self = Self(0x140a);
    pub const GL_INT: Self = Self(0x1404);
    pub const GL_BYTE: Self = Self(0x1400);
    pub const GL_SHORT: Self = Self(0x1402);
    pub const GL_FLOAT: Self = Self(0x1406);
    pub const GL_DOUBLE_EXT: Self = Self(0x140a);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MemoryObjectParameterName(pub(crate) u64);
impl MemoryObjectParameterName {
    pub const GL_DEDICATED_MEMORY_OBJECT_EXT: Self = Self(0x9581);
    pub const GL_PROTECTED_MEMORY_OBJECT_EXT: Self = Self(0x959b);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ColorTableParameterPName(pub(crate) u64);
impl ColorTableParameterPName {
    pub const GL_COLOR_TABLE_SCALE: Self = Self(0x80d6);
    pub const GL_COLOR_TABLE_WIDTH: Self = Self(0x80d9);
    pub const GL_COLOR_TABLE_BLUE_SIZE: Self = Self(0x80dc);
    pub const GL_COLOR_TABLE_BIAS: Self = Self(0x80d7);
    pub const GL_COLOR_TABLE_INTENSITY_SIZE: Self = Self(0x80df);
    pub const GL_COLOR_TABLE_GREEN_SIZE: Self = Self(0x80db);
    pub const GL_COLOR_TABLE_FORMAT: Self = Self(0x80d8);
    pub const GL_COLOR_TABLE_LUMINANCE_SIZE: Self = Self(0x80de);
    pub const GL_COLOR_TABLE_RED_SIZE: Self = Self(0x80da);
    pub const GL_COLOR_TABLE_ALPHA_SIZE: Self = Self(0x80dd);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TexStorageAttribss(pub(crate) u64);
impl TexStorageAttribss {
    pub const GL_SURFACE_COMPRESSION_FIXED_RATE_NONE_EXT: Self = Self(0x96c1);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ContainerType(pub(crate) u64);
impl ContainerType {
    pub const GL_PROGRAM_OBJECT_ARB: Self = Self(0x8b40);
    pub const GL_PROGRAM_OBJECT_EXT: Self = Self(0x8b40);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VertexBufferObjectUsage(pub(crate) u64);
impl VertexBufferObjectUsage {
    pub const GL_STREAM_COPY: Self = Self(0x88e2);
    pub const GL_STATIC_DRAW: Self = Self(0x88e4);
    pub const GL_DYNAMIC_READ: Self = Self(0x88e9);
    pub const GL_DYNAMIC_COPY: Self = Self(0x88ea);
    pub const GL_STATIC_COPY: Self = Self(0x88e6);
    pub const GL_STATIC_READ: Self = Self(0x88e5);
    pub const GL_DYNAMIC_DRAW: Self = Self(0x88e8);
    pub const GL_STREAM_READ: Self = Self(0x88e1);
    pub const GL_STREAM_DRAW: Self = Self(0x88e0);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GetPixelMap(pub(crate) u64);
impl GetPixelMap {
    pub const GL_PIXEL_MAP_B_TO_B: Self = Self(0x0c78);
    pub const GL_PIXEL_MAP_A_TO_A: Self = Self(0x0c79);
    pub const GL_PIXEL_MAP_I_TO_R: Self = Self(0x0c72);
    pub const GL_PIXEL_MAP_I_TO_A: Self = Self(0x0c75);
    pub const GL_PIXEL_MAP_I_TO_B: Self = Self(0x0c74);
    pub const GL_PIXEL_MAP_I_TO_I: Self = Self(0x0c70);
    pub const GL_PIXEL_MAP_G_TO_G: Self = Self(0x0c77);
    pub const GL_PIXEL_MAP_R_TO_R: Self = Self(0x0c76);
    pub const GL_PIXEL_MAP_S_TO_S: Self = Self(0x0c71);
    pub const GL_PIXEL_MAP_I_TO_G: Self = Self(0x0c73);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PixelFormat(pub(crate) u64);
impl PixelFormat {
    pub const GL_COLOR_INDEX: Self = Self(0x1900);
    pub const GL_BGRA_INTEGER: Self = Self(0x8d9b);
    pub const GL_RGBA: Self = Self(0x1908);
    pub const GL_DEPTH_STENCIL: Self = Self(0x84f9);
    pub const GL_DEPTH_COMPONENT: Self = Self(0x1902);
    pub const GL_RED_INTEGER: Self = Self(0x8d94);
    pub const GL_RED: Self = Self(0x1903);
    pub const GL_RGB_INTEGER: Self = Self(0x8d98);
    pub const GL_UNSIGNED_SHORT: Self = Self(0x1403);
    pub const GL_BLUE_INTEGER: Self = Self(0x8d96);
    pub const GL_RGB: Self = Self(0x1907);
    pub const GL_UNSIGNED_INT: Self = Self(0x1405);
    pub const GL_GREEN: Self = Self(0x1904);
    pub const GL_ABGR_EXT: Self = Self(0x8000);
    pub const GL_LUMINANCE: Self = Self(0x1909);
    pub const GL_GREEN_INTEGER: Self = Self(0x8d95);
    pub const GL_LUMINANCE_ALPHA: Self = Self(0x190a);
    pub const GL_YCRCB_444_SGIX: Self = Self(0x81bc);
    pub const GL_RED_EXT: Self = Self(0x1903);
    pub const GL_YCRCB_422_SGIX: Self = Self(0x81bb);
    pub const GL_CMYKA_EXT: Self = Self(0x800d);
    pub const GL_RG_INTEGER: Self = Self(0x8228);
    pub const GL_BGR_INTEGER: Self = Self(0x8d9a);
    pub const GL_STENCIL_INDEX: Self = Self(0x1901);
    pub const GL_RG: Self = Self(0x8227);
    pub const GL_ALPHA: Self = Self(0x1906);
    pub const GL_CMYK_EXT: Self = Self(0x800c);
    pub const GL_BGR: Self = Self(0x80e0);
    pub const GL_BLUE: Self = Self(0x1905);
    pub const GL_BGRA: Self = Self(0x80e1);
    pub const GL_RGBA_INTEGER: Self = Self(0x8d99);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct UniformBlockPName(pub(crate) u64);
impl UniformBlockPName {
    pub const GL_UNIFORM_BLOCK_REFERENCED_BY_TESS_CONTROL_SHADER: Self = Self(0x84f0);
    pub const GL_UNIFORM_BLOCK_REFERENCED_BY_COMPUTE_SHADER: Self = Self(0x90ec);
    pub const GL_UNIFORM_BLOCK_REFERENCED_BY_FRAGMENT_SHADER: Self = Self(0x8a46);
    pub const GL_UNIFORM_BLOCK_BINDING: Self = Self(0x8a3f);
    pub const GL_UNIFORM_BLOCK_ACTIVE_UNIFORM_INDICES: Self = Self(0x8a43);
    pub const GL_UNIFORM_BLOCK_REFERENCED_BY_GEOMETRY_SHADER: Self = Self(0x8a45);
    pub const GL_UNIFORM_BLOCK_REFERENCED_BY_TESS_EVALUATION_SHADER: Self = Self(0x84f1);
    pub const GL_UNIFORM_BLOCK_DATA_SIZE: Self = Self(0x8a40);
    pub const GL_UNIFORM_BLOCK_REFERENCED_BY_VERTEX_SHADER: Self = Self(0x8a44);
    pub const GL_UNIFORM_BLOCK_NAME_LENGTH: Self = Self(0x8a41);
    pub const GL_UNIFORM_BLOCK_ACTIVE_UNIFORMS: Self = Self(0x8a42);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PixelTexGenParameterNameSGIS(pub(crate) u64);
impl PixelTexGenParameterNameSGIS {
    pub const GL_PIXEL_FRAGMENT_RGB_SOURCE_SGIS: Self = Self(0x8354);
    pub const GL_PIXEL_FRAGMENT_ALPHA_SOURCE_SGIS: Self = Self(0x8355);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GetMultisamplePNameNV(pub(crate) u64);
impl GetMultisamplePNameNV {
    pub const GL_SAMPLE_LOCATION_ARB: Self = Self(0x8e50);
    pub const GL_PROGRAMMABLE_SAMPLE_LOCATION_ARB: Self = Self(0x9341);
    pub const GL_SAMPLE_POSITION: Self = Self(0x8e50);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MatrixIndexPointerTypeARB(pub(crate) u64);
impl MatrixIndexPointerTypeARB {
    pub const GL_UNSIGNED_BYTE: Self = Self(0x1401);
    pub const GL_UNSIGNED_INT: Self = Self(0x1405);
    pub const GL_UNSIGNED_SHORT: Self = Self(0x1403);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SizedInternalFormat(pub(crate) u64);
impl SizedInternalFormat {
    pub const GL_DEPTH_COMPONENT32_ARB: Self = Self(0x81a7);
    pub const GL_RGB32UI_EXT: Self = Self(0x8d71);
    pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_12x10: Self = Self(0x93dc);
    pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_10x6: Self = Self(0x93d9);
    pub const GL_RG8: Self = Self(0x822b);
    pub const GL_LUMINANCE8UI_EXT: Self = Self(0x8d80);
    pub const GL_COMPRESSED_RGB_BPTC_SIGNED_FLOAT_EXT: Self = Self(0x8e8e);
    pub const GL_RGB5: Self = Self(0x8050);
    pub const GL_STENCIL_INDEX8_OES: Self = Self(0x8d48);
    pub const GL_RG32F_EXT: Self = Self(0x8230);
    pub const GL_COMPRESSED_RGBA_S3TC_DXT3_EXT: Self = Self(0x83f2);
    pub const GL_ETC1_RGB8_OES: Self = Self(0x8d64);
    pub const GL_COMPRESSED_RGBA_ASTC_10x6_KHR: Self = Self(0x93b9);
    pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_8x5: Self = Self(0x93d5);
    pub const GL_LUMINANCE8I_EXT: Self = Self(0x8d92);
    pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_10x5_KHR: Self = Self(0x93d8);
    pub const GL_INTENSITY12_EXT: Self = Self(0x804c);
    pub const GL_RGB32F_ARB: Self = Self(0x8815);
    pub const GL_INTENSITY32UI_EXT: Self = Self(0x8d73);
    pub const GL_R16I: Self = Self(0x8233);
    pub const GL_RGBA32I: Self = Self(0x8d82);
    pub const GL_R16: Self = Self(0x822a);
    pub const GL_RGB12_EXT: Self = Self(0x8053);
    pub const GL_DEPTH24_STENCIL8: Self = Self(0x88f0);
    pub const GL_RGB565_OES: Self = Self(0x8d62);
    pub const GL_COMPRESSED_RGBA_S3TC_DXT1_EXT: Self = Self(0x83f1);
    pub const GL_RGBA2_EXT: Self = Self(0x8055);
    pub const GL_RGB32I: Self = Self(0x8d83);
    pub const GL_COMPRESSED_RGBA_ASTC_8x6: Self = Self(0x93b6);
    pub const GL_RGBA16F_ARB: Self = Self(0x881a);
    pub const GL_RGB10_A2UI: Self = Self(0x906f);
    pub const GL_DEPTH_COMPONENT16_SGIX: Self = Self(0x81a5);
    pub const GL_LUMINANCE4_ALPHA4: Self = Self(0x8043);
    pub const GL_ALPHA16: Self = Self(0x803e);
    pub const GL_RGB4: Self = Self(0x804f);
    pub const GL_LUMINANCE12_ALPHA12: Self = Self(0x8047);
    pub const GL_R8_SNORM: Self = Self(0x8f94);
    pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_6x6: Self = Self(0x93d4);
    pub const GL_LUMINANCE6_ALPHA2_EXT: Self = Self(0x8044);
    pub const GL_R16F: Self = Self(0x822d);
    pub const GL_COMPRESSED_SRGB8_ETC2_OES: Self = Self(0x9275);
    pub const GL_RGB8UI: Self = Self(0x8d7d);
    pub const GL_COMPRESSED_RGBA_ASTC_6x5x5_OES: Self = Self(0x93c7);
    pub const GL_COMPRESSED_SRGB_ALPHA_S3TC_DXT3_EXT: Self = Self(0x8c4e);
    pub const GL_COMPRESSED_SIGNED_R11_EAC_OES: Self = Self(0x9271);
    pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_5x5_KHR: Self = Self(0x93d2);
    pub const GL_R8UI: Self = Self(0x8232);
    pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_12x12: Self = Self(0x93dd);
    pub const GL_COMPRESSED_RGBA_ASTC_12x10_KHR: Self = Self(0x93bc);
    pub const GL_ALPHA12_EXT: Self = Self(0x803d);
    pub const GL_RGBA4_EXT: Self = Self(0x8056);
    pub const GL_DEPTH_COMPONENT16_OES: Self = Self(0x81a5);
    pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_5x5x5_OES: Self = Self(0x93e6);
    pub const GL_RG8_SNORM: Self = Self(0x8f95);
    pub const GL_RG8UI: Self = Self(0x8238);
    pub const GL_COMPRESSED_SRGB_ALPHA_BPTC_UNORM: Self = Self(0x8e8d);
    pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_5x4x4_OES: Self = Self(0x93e4);
    pub const GL_COMPRESSED_SRGB8_ALPHA8_ETC2_EAC: Self = Self(0x9279);
    pub const GL_R32F_EXT: Self = Self(0x822e);
    pub const GL_RG16I: Self = Self(0x8239);
    pub const GL_RG32I: Self = Self(0x823b);
    pub const GL_ALPHA16I_EXT: Self = Self(0x8d8a);
    pub const GL_RGBA12: Self = Self(0x805a);
    pub const GL_RGB8I_EXT: Self = Self(0x8d8f);
    pub const GL_COMPRESSED_SIGNED_RG_RGTC2: Self = Self(0x8dbe);
    pub const GL_RGBA16_SNORM: Self = Self(0x8f9b);
    pub const GL_DEPTH_COMPONENT16_ARB: Self = Self(0x81a5);
    pub const GL_COMPRESSED_RGBA_ASTC_8x8_KHR: Self = Self(0x93b7);
    pub const GL_COMPRESSED_RGBA_ASTC_4x4x3_OES: Self = Self(0x93c2);
    pub const GL_COMPRESSED_RGBA_ASTC_10x10_KHR: Self = Self(0x93bb);
    pub const GL_STENCIL_INDEX4_OES: Self = Self(0x8d47);
    pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_8x8_KHR: Self = Self(0x93d7);
    pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_4x4x4_OES: Self = Self(0x93e3);
    pub const GL_COMPRESSED_R11_EAC: Self = Self(0x9270);
    pub const GL_LUMINANCE16_ALPHA16_EXT: Self = Self(0x8048);
    pub const GL_LUMINANCE16_ALPHA16: Self = Self(0x8048);
    pub const GL_INTENSITY8UI_EXT: Self = Self(0x8d7f);
    pub const GL_COMPRESSED_RGB_S3TC_DXT1_EXT: Self = Self(0x83f0);
    pub const GL_R8: Self = Self(0x8229);
    pub const GL_RGBA32F: Self = Self(0x8814);
    pub const GL_LUMINANCE_ALPHA32UI_EXT: Self = Self(0x8d75);
    pub const GL_R8I: Self = Self(0x8231);
    pub const GL_SRGB8_ALPHA8_EXT: Self = Self(0x8c43);
    pub const GL_INTENSITY16I_EXT: Self = Self(0x8d8b);
    pub const GL_COMPRESSED_RGB_BPTC_SIGNED_FLOAT: Self = Self(0x8e8e);
    pub const GL_COMPRESSED_RGB8_ETC2_OES: Self = Self(0x9274);
    pub const GL_STENCIL_INDEX4: Self = Self(0x8d47);
    pub const GL_COMPRESSED_RGBA_ASTC_5x4_KHR: Self = Self(0x93b1);
    pub const GL_R32I: Self = Self(0x8235);
    pub const GL_LUMINANCE4_ALPHA4_OES: Self = Self(0x8043);
    pub const GL_COMPRESSED_RGBA_ASTC_12x12_KHR: Self = Self(0x93bd);
    pub const GL_ALPHA4_EXT: Self = Self(0x803b);
    pub const GL_RGB16_SNORM_EXT: Self = Self(0x8f9a);
    pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_8x6: Self = Self(0x93d6);
    pub const GL_COMPRESSED_RED_GREEN_RGTC2_EXT: Self = Self(0x8dbd);
    pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_10x10_KHR: Self = Self(0x93db);
    pub const GL_ALPHA8I_EXT: Self = Self(0x8d90);
    pub const GL_RGBA16F_EXT: Self = Self(0x881a);
    pub const GL_COMPRESSED_RGB_BPTC_UNSIGNED_FLOAT: Self = Self(0x8e8f);
    pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_12x12_KHR: Self = Self(0x93dd);
    pub const GL_COMPRESSED_RGBA_S3TC_DXT5_EXT: Self = Self(0x83f3);
    pub const GL_RGB8_OES: Self = Self(0x8051);
    pub const GL_RGB16I_EXT: Self = Self(0x8d89);
    pub const GL_COMPRESSED_RGBA_ASTC_10x5: Self = Self(0x93b8);
    pub const GL_COMPRESSED_SRGB_ALPHA_S3TC_DXT1_NV: Self = Self(0x8c4d);
    pub const GL_RGB5_A1_EXT: Self = Self(0x8057);
    pub const GL_DEPTH_COMPONENT32F: Self = Self(0x8cac);
    pub const GL_COMPRESSED_SRGB_ALPHA_BPTC_UNORM_ARB: Self = Self(0x8e8d);
    pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_6x5_KHR: Self = Self(0x93d3);
    pub const GL_LUMINANCE4: Self = Self(0x803f);
    pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_5x5x4_OES: Self = Self(0x93e5);
    pub const GL_LUMINANCE_ALPHA16UI_EXT: Self = Self(0x8d7b);
    pub const GL_RG8_EXT: Self = Self(0x822b);
    pub const GL_INTENSITY4_EXT: Self = Self(0x804a);
    pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_12x10_KHR: Self = Self(0x93dc);
    pub const GL_LUMINANCE4_EXT: Self = Self(0x803f);
    pub const GL_RGB9_E5_EXT: Self = Self(0x8c3d);
    pub const GL_SRGB8: Self = Self(0x8c41);
    pub const GL_COMPRESSED_SRGB8_ALPHA8_ETC2_EAC_OES: Self = Self(0x9279);
    pub const GL_ALPHA8: Self = Self(0x803c);
    pub const GL_RGBA4_OES: Self = Self(0x8056);
    pub const GL_RGBA8_OES: Self = Self(0x8058);
    pub const GL_R16_EXT: Self = Self(0x822a);
    pub const GL_DEPTH32F_STENCIL8_NV: Self = Self(0x8dac);
    pub const GL_COMPRESSED_SRGB8_PUNCHTHROUGH_ALPHA1_ETC2_OES: Self = Self(0x9277);
    pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_4x4_KHR: Self = Self(0x93d0);
    pub const GL_LUMINANCE8_EXT: Self = Self(0x8040);
    pub const GL_COMPRESSED_RGBA_ASTC_6x5_KHR: Self = Self(0x93b3);
    pub const GL_COMPRESSED_RGBA_ASTC_10x10: Self = Self(0x93bb);
    pub const GL_RGB8: Self = Self(0x8051);
    pub const GL_RGB565: Self = Self(0x8d62);
    pub const GL_COMPRESSED_SRGB_S3TC_DXT1_NV: Self = Self(0x8c4c);
    pub const GL_INTENSITY16UI_EXT: Self = Self(0x8d79);
    pub const GL_COMPRESSED_SIGNED_RED_RGTC1_EXT: Self = Self(0x8dbc);
    pub const GL_RGBA8UI_EXT: Self = Self(0x8d7c);
    pub const GL_COMPRESSED_RGB8_PUNCHTHROUGH_ALPHA1_ETC2: Self = Self(0x9276);
    pub const GL_COMPRESSED_SRGB8_ETC2: Self = Self(0x9275);
    pub const GL_LUMINANCE8_ALPHA8_EXT: Self = Self(0x8045);
    pub const GL_COMPRESSED_RGBA8_ETC2_EAC: Self = Self(0x9278);
    pub const GL_RGB16: Self = Self(0x8054);
    pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_8x8: Self = Self(0x93d7);
    pub const GL_COMPRESSED_RGBA_ASTC_6x5: Self = Self(0x93b3);
    pub const GL_COMPRESSED_RGBA_ASTC_5x5x5_OES: Self = Self(0x93c6);
    pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_8x5_KHR: Self = Self(0x93d5);
    pub const GL_RGBA12_EXT: Self = Self(0x805a);
    pub const GL_COMPRESSED_RGBA_ASTC_12x10: Self = Self(0x93bc);
    pub const GL_R8_EXT: Self = Self(0x8229);
    pub const GL_ALPHA4: Self = Self(0x803b);
    pub const GL_RG16F_EXT: Self = Self(0x822f);
    pub const GL_ALPHA8_EXT: Self = Self(0x803c);
    pub const GL_COMPRESSED_R11_EAC_OES: Self = Self(0x9270);
    pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_6x5x5_OES: Self = Self(0x93e7);
    pub const GL_COMPRESSED_RGB_BPTC_SIGNED_FLOAT_ARB: Self = Self(0x8e8e);
    pub const GL_DEPTH24_STENCIL8_OES: Self = Self(0x88f0);
    pub const GL_RGB16UI: Self = Self(0x8d77);
    pub const GL_RGBA16I_EXT: Self = Self(0x8d88);
    pub const GL_COMPRESSED_RGBA_BPTC_UNORM_EXT: Self = Self(0x8e8c);
    pub const GL_RGBA8_SNORM: Self = Self(0x8f97);
    pub const GL_RGBA8: Self = Self(0x8058);
    pub const GL_COMPRESSED_RGBA_ASTC_8x5: Self = Self(0x93b5);
    pub const GL_COMPRESSED_RGBA_ASTC_6x6x6_OES: Self = Self(0x93c9);
    pub const GL_LUMINANCE8_OES: Self = Self(0x8040);
    pub const GL_RGB16F_ARB: Self = Self(0x881b);
    pub const GL_R11F_G11F_B10F: Self = Self(0x8c3a);
    pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_10x5: Self = Self(0x93d8);
    pub const GL_RGBA4: Self = Self(0x8056);
    pub const GL_RGB32F_EXT: Self = Self(0x8815);
    pub const GL_RGB10_A2: Self = Self(0x8059);
    pub const GL_DEPTH_COMPONENT24_SGIX: Self = Self(0x81a6);
    pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_6x6_KHR: Self = Self(0x93d4);
    pub const GL_COMPRESSED_RGBA_ASTC_4x4: Self = Self(0x93b0);
    pub const GL_LUMINANCE4_ALPHA4_EXT: Self = Self(0x8043);
    pub const GL_DEPTH_COMPONENT32F_NV: Self = Self(0x8dab);
    pub const GL_COMPRESSED_SIGNED_R11_EAC: Self = Self(0x9271);
    pub const GL_RGB16F: Self = Self(0x881b);
    pub const GL_RGBA32I_EXT: Self = Self(0x8d82);
    pub const GL_RGB5_EXT: Self = Self(0x8050);
    pub const GL_DEPTH_COMPONENT32_SGIX: Self = Self(0x81a7);
    pub const GL_LUMINANCE_ALPHA32I_EXT: Self = Self(0x8d87);
    pub const GL_COMPRESSED_RGBA_S3TC_DXT3_ANGLE: Self = Self(0x83f2);
    pub const GL_R16_SNORM: Self = Self(0x8f98);
    pub const GL_COMPRESSED_RGB8_ETC2: Self = Self(0x9274);
    pub const GL_COMPRESSED_RGB_BPTC_UNSIGNED_FLOAT_ARB: Self = Self(0x8e8f);
    pub const GL_DEPTH_COMPONENT32: Self = Self(0x81a7);
    pub const GL_RGBA8UI: Self = Self(0x8d7c);
    pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_6x6x5_OES: Self = Self(0x93e8);
    pub const GL_RGBA16I: Self = Self(0x8d88);
    pub const GL_COMPRESSED_RED_RGTC1_EXT: Self = Self(0x8dbb);
    pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_4x4x3_OES: Self = Self(0x93e2);
    pub const GL_LUMINANCE_ALPHA8I_EXT: Self = Self(0x8d93);
    pub const GL_COMPRESSED_RED_RGTC1: Self = Self(0x8dbb);
    pub const GL_RGB8I: Self = Self(0x8d8f);
    pub const GL_COMPRESSED_RGBA_ASTC_4x3x3_OES: Self = Self(0x93c1);
    pub const GL_INTENSITY12: Self = Self(0x804c);
    pub const GL_LUMINANCE12_ALPHA4: Self = Self(0x8046);
    pub const GL_RGB2_EXT: Self = Self(0x804e);
    pub const GL_INTENSITY4: Self = Self(0x804a);
    pub const GL_COMPRESSED_SRGB_ALPHA_S3TC_DXT5_EXT: Self = Self(0x8c4f);
    pub const GL_COMPRESSED_RGBA_ASTC_5x4x4_OES: Self = Self(0x93c4);
    pub const GL_COMPRESSED_RGBA_ASTC_6x6_KHR: Self = Self(0x93b4);
    pub const GL_RGB4_EXT: Self = Self(0x804f);
    pub const GL_R32F: Self = Self(0x822e);
    pub const GL_R32UI: Self = Self(0x8236);
    pub const GL_COMPRESSED_RGBA_ASTC_5x5x4_OES: Self = Self(0x93c5);
    pub const GL_LUMINANCE8_ALPHA8: Self = Self(0x8045);
    pub const GL_R16F_EXT: Self = Self(0x822d);
    pub const GL_LUMINANCE12_EXT: Self = Self(0x8041);
    pub const GL_STENCIL_INDEX1_EXT: Self = Self(0x8d46);
    pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_8x6_KHR: Self = Self(0x93d6);
    pub const GL_RG16UI: Self = Self(0x823a);
    pub const GL_RG16_SNORM_EXT: Self = Self(0x8f99);
    pub const GL_RGBA8_EXT: Self = Self(0x8058);
    pub const GL_COMPRESSED_SRGB_ALPHA_S3TC_DXT3_NV: Self = Self(0x8c4e);
    pub const GL_RGBA16_SNORM_EXT: Self = Self(0x8f9b);
    pub const GL_LUMINANCE16_EXT: Self = Self(0x8042);
    pub const GL_RGBA16: Self = Self(0x805b);
    pub const GL_RGB32UI: Self = Self(0x8d71);
    pub const GL_COMPRESSED_SRGB8_PUNCHTHROUGH_ALPHA1_ETC2: Self = Self(0x9277);
    pub const GL_COMPRESSED_RGBA_ASTC_5x4: Self = Self(0x93b1);
    pub const GL_LUMINANCE32I_EXT: Self = Self(0x8d86);
    pub const GL_ALPHA8UI_EXT: Self = Self(0x8d7e);
    pub const GL_COMPRESSED_SIGNED_RED_RGTC1: Self = Self(0x8dbc);
    pub const GL_RGB5_A1_OES: Self = Self(0x8057);
    pub const GL_COMPRESSED_RG11_EAC_OES: Self = Self(0x9272);
    pub const GL_DEPTH_COMPONENT24: Self = Self(0x81a6);
    pub const GL_COMPRESSED_SRGB_ALPHA_S3TC_DXT5_NV: Self = Self(0x8c4f);
    pub const GL_STENCIL_INDEX16_EXT: Self = Self(0x8d49);
    pub const GL_COMPRESSED_RG11_EAC: Self = Self(0x9272);
    pub const GL_RG8I: Self = Self(0x8237);
    pub const GL_INTENSITY8_EXT: Self = Self(0x804b);
    pub const GL_RGB5_A1: Self = Self(0x8057);
    pub const GL_R16UI: Self = Self(0x8234);
    pub const GL_LUMINANCE_ALPHA16I_EXT: Self = Self(0x8d8d);
    pub const GL_RG16_EXT: Self = Self(0x822c);
    pub const GL_RGBA8I_EXT: Self = Self(0x8d8e);
    pub const GL_ALPHA12: Self = Self(0x803d);
    pub const GL_COMPRESSED_SRGB_ALPHA_BPTC_UNORM_EXT: Self = Self(0x8e8d);
    pub const GL_SRGB8_EXT: Self = Self(0x8c41);
    pub const GL_STENCIL_INDEX8: Self = Self(0x8d48);
    pub const GL_RG16_SNORM: Self = Self(0x8f99);
    pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_6x5: Self = Self(0x93d3);
    pub const GL_STENCIL_INDEX8_EXT: Self = Self(0x8d48);
    pub const GL_RGB32I_EXT: Self = Self(0x8d83);
    pub const GL_SRGB8_NV: Self = Self(0x8c41);
    pub const GL_RGBA32UI_EXT: Self = Self(0x8d70);
    pub const GL_LUMINANCE12_ALPHA4_EXT: Self = Self(0x8046);
    pub const GL_STENCIL_INDEX1_OES: Self = Self(0x8d46);
    pub const GL_RGBA16_EXT: Self = Self(0x805b);
    pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_10x10: Self = Self(0x93db);
    pub const GL_RGB16F_EXT: Self = Self(0x881b);
    pub const GL_DEPTH_COMPONENT24_ARB: Self = Self(0x81a6);
    pub const GL_DEPTH32F_STENCIL8: Self = Self(0x8cad);
    pub const GL_LUMINANCE16: Self = Self(0x8042);
    pub const GL_RGB9_E5_APPLE: Self = Self(0x8c3d);
    pub const GL_COMPRESSED_RGBA_ASTC_10x5_KHR: Self = Self(0x93b8);
    pub const GL_RGB32F: Self = Self(0x8815);
    pub const GL_R11F_G11F_B10F_APPLE: Self = Self(0x8c3a);
    pub const GL_LUMINANCE32UI_EXT: Self = Self(0x8d74);
    pub const GL_RGB16I: Self = Self(0x8d89);
    pub const GL_RGB10: Self = Self(0x8052);
    pub const GL_COMPRESSED_RGBA_ASTC_3x3x3_OES: Self = Self(0x93c0);
    pub const GL_COMPRESSED_RGBA_ASTC_5x5: Self = Self(0x93b2);
    pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_10x8: Self = Self(0x93da);
    pub const GL_RGB8_EXT: Self = Self(0x8051);
    pub const GL_COMPRESSED_SRGB_S3TC_DXT1_EXT: Self = Self(0x8c4c);
    pub const GL_RGBA16F: Self = Self(0x881a);
    pub const GL_ALPHA8_OES: Self = Self(0x803c);
    pub const GL_RGB10_A2_EXT: Self = Self(0x8059);
    pub const GL_LUMINANCE_ALPHA8UI_EXT: Self = Self(0x8d81);
    pub const GL_ALPHA32I_EXT: Self = Self(0x8d84);
    pub const GL_RGBA32F_EXT: Self = Self(0x8814);
    pub const GL_COMPRESSED_RGBA_ASTC_8x5_KHR: Self = Self(0x93b5);
    pub const GL_COMPRESSED_RGB8_PUNCHTHROUGH_ALPHA1_ETC2_OES: Self = Self(0x9276);
    pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_5x4: Self = Self(0x93d1);
    pub const GL_RGBA32UI: Self = Self(0x8d70);
    pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_10x8_KHR: Self = Self(0x93da);
    pub const GL_RGB10_EXT: Self = Self(0x8052);
    pub const GL_COMPRESSED_RGBA_ASTC_5x5_KHR: Self = Self(0x93b2);
    pub const GL_LUMINANCE16UI_EXT: Self = Self(0x8d7a);
    pub const GL_RG32UI: Self = Self(0x823c);
    pub const GL_COMPRESSED_RGBA_ASTC_6x6: Self = Self(0x93b4);
    pub const GL_R3_G3_B2: Self = Self(0x2a10);
    pub const GL_DEPTH_COMPONENT24_OES: Self = Self(0x81a6);
    pub const GL_RGB9_E5: Self = Self(0x8c3d);
    pub const GL_COMPRESSED_RGB_BPTC_UNSIGNED_FLOAT_EXT: Self = Self(0x8e8f);
    pub const GL_COMPRESSED_RGBA_ASTC_4x4x4_OES: Self = Self(0x93c3);
    pub const GL_DEPTH_COMPONENT16: Self = Self(0x81a5);
    pub const GL_RG16: Self = Self(0x822c);
    pub const GL_LUMINANCE8: Self = Self(0x8040);
    pub const GL_LUMINANCE12: Self = Self(0x8041);
    pub const GL_RGB16_SNORM: Self = Self(0x8f9a);
    pub const GL_COMPRESSED_RGBA_ASTC_10x8_KHR: Self = Self(0x93ba);
    pub const GL_COMPRESSED_RGBA_ASTC_4x4_KHR: Self = Self(0x93b0);
    pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_5x4_KHR: Self = Self(0x93d1);
    pub const GL_RGB8_SNORM: Self = Self(0x8f96);
    pub const GL_DEPTH_COMPONENT32_OES: Self = Self(0x81a7);
    pub const GL_RG16F: Self = Self(0x822f);
    pub const GL_RGBA16UI: Self = Self(0x8d76);
    pub const GL_COMPRESSED_RGBA_ASTC_10x6: Self = Self(0x93b9);
    pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_4x3x3_OES: Self = Self(0x93e1);
    pub const GL_LUMINANCE12_ALPHA12_EXT: Self = Self(0x8047);
    pub const GL_INTENSITY16: Self = Self(0x804d);
    pub const GL_COMPRESSED_RGBA_ASTC_6x6x5_OES: Self = Self(0x93c8);
    pub const GL_COMPRESSED_SRGB_ALPHA_S3TC_DXT1_EXT: Self = Self(0x8c4d);
    pub const GL_STENCIL_INDEX1: Self = Self(0x8d46);
    pub const GL_STENCIL_INDEX4_EXT: Self = Self(0x8d47);
    pub const GL_RGBA16UI_EXT: Self = Self(0x8d76);
    pub const GL_COMPRESSED_RGBA_BPTC_UNORM: Self = Self(0x8e8c);
    pub const GL_STENCIL_INDEX16: Self = Self(0x8d49);
    pub const GL_COMPRESSED_RGBA_BPTC_UNORM_ARB: Self = Self(0x8e8c);
    pub const GL_DEPTH24_STENCIL8_EXT: Self = Self(0x88f0);
    pub const GL_COMPRESSED_SIGNED_RG11_EAC_OES: Self = Self(0x9273);
    pub const GL_R16_SNORM_EXT: Self = Self(0x8f98);
    pub const GL_ALPHA16_EXT: Self = Self(0x803e);
    pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_3x3x3_OES: Self = Self(0x93e0);
    pub const GL_COMPRESSED_SIGNED_RG11_EAC: Self = Self(0x9273);
    pub const GL_LUMINANCE8_ALPHA8_OES: Self = Self(0x8045);
    pub const GL_COMPRESSED_RGBA8_ETC2_EAC_OES: Self = Self(0x9278);
    pub const GL_RGB12: Self = Self(0x8053);
    pub const GL_LUMINANCE6_ALPHA2: Self = Self(0x8044);
    pub const GL_COMPRESSED_RGBA_ASTC_10x8: Self = Self(0x93ba);
    pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_6x6x6_OES: Self = Self(0x93e9);
    pub const GL_RGB16UI_EXT: Self = Self(0x8d77);
    pub const GL_SRGB8_ALPHA8: Self = Self(0x8c43);
    pub const GL_COMPRESSED_RGBA_ASTC_8x8: Self = Self(0x93b7);
    pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_5x5: Self = Self(0x93d2);
    pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_10x6_KHR: Self = Self(0x93d9);
    pub const GL_INTENSITY8: Self = Self(0x804b);
    pub const GL_ALPHA32UI_EXT: Self = Self(0x8d72);
    pub const GL_ALPHA16UI_EXT: Self = Self(0x8d78);
    pub const GL_INTENSITY8I_EXT: Self = Self(0x8d91);
    pub const GL_RGB16_EXT: Self = Self(0x8054);
    pub const GL_LUMINANCE16I_EXT: Self = Self(0x8d8c);
    pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_4x4: Self = Self(0x93d0);
    pub const GL_COMPRESSED_RGBA_ASTC_8x6_KHR: Self = Self(0x93b6);
    pub const GL_INTENSITY16_EXT: Self = Self(0x804d);
    pub const GL_R11F_G11F_B10F_EXT: Self = Self(0x8c3a);
    pub const GL_RGBA8I: Self = Self(0x8d8e);
    pub const GL_COMPRESSED_RGBA_S3TC_DXT5_ANGLE: Self = Self(0x83f3);
    pub const GL_INTENSITY32I_EXT: Self = Self(0x8d85);
    pub const GL_COMPRESSED_RG_RGTC2: Self = Self(0x8dbd);
    pub const GL_RG32F: Self = Self(0x8230);
    pub const GL_RGB8UI_EXT: Self = Self(0x8d7d);
    pub const GL_RGBA2: Self = Self(0x8055);
    pub const GL_RGBA32F_ARB: Self = Self(0x8814);
    pub const GL_COMPRESSED_SIGNED_RED_GREEN_RGTC2_EXT: Self = Self(0x8dbe);
    pub const GL_COMPRESSED_RGBA_ASTC_12x12: Self = Self(0x93bd);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BufferTargetARB(pub(crate) u64);
impl BufferTargetARB {
    pub const GL_TEXTURE_BUFFER: Self = Self(0x8c2a);
    pub const GL_TRANSFORM_FEEDBACK_BUFFER: Self = Self(0x8c8e);
    pub const GL_PIXEL_PACK_BUFFER: Self = Self(0x88eb);
    pub const GL_DRAW_INDIRECT_BUFFER: Self = Self(0x8f3f);
    pub const GL_DISPATCH_INDIRECT_BUFFER: Self = Self(0x90ee);
    pub const GL_PIXEL_UNPACK_BUFFER: Self = Self(0x88ec);
    pub const GL_UNIFORM_BUFFER: Self = Self(0x8a11);
    pub const GL_COPY_READ_BUFFER: Self = Self(0x8f36);
    pub const GL_PARAMETER_BUFFER: Self = Self(0x80ee);
    pub const GL_COPY_WRITE_BUFFER: Self = Self(0x8f37);
    pub const GL_ATOMIC_COUNTER_BUFFER: Self = Self(0x92c0);
    pub const GL_QUERY_BUFFER: Self = Self(0x9192);
    pub const GL_ARRAY_BUFFER: Self = Self(0x8892);
    pub const GL_SHADER_STORAGE_BUFFER: Self = Self(0x90d2);
    pub const GL_ELEMENT_ARRAY_BUFFER: Self = Self(0x8893);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ListNameType(pub(crate) u64);
impl ListNameType {
    pub const GL_UNSIGNED_SHORT: Self = Self(0x1403);
    pub const GL_BYTE: Self = Self(0x1400);
    pub const GL_FLOAT: Self = Self(0x1406);
    pub const GL_UNSIGNED_BYTE: Self = Self(0x1401);
    pub const GL_INT: Self = Self(0x1404);
    pub const GL_UNSIGNED_INT: Self = Self(0x1405);
    pub const GL_2_BYTES: Self = Self(0x1407);
    pub const GL_4_BYTES: Self = Self(0x1409);
    pub const GL_SHORT: Self = Self(0x1402);
    pub const GL_3_BYTES: Self = Self(0x1408);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GetVariantValueEXT(pub(crate) u64);
impl GetVariantValueEXT {
    pub const GL_VARIANT_DATATYPE_EXT: Self = Self(0x87e5);
    pub const GL_VARIANT_ARRAY_TYPE_EXT: Self = Self(0x87e7);
    pub const GL_VARIANT_ARRAY_STRIDE_EXT: Self = Self(0x87e6);
    pub const GL_VARIANT_VALUE_EXT: Self = Self(0x87e4);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VertexPointerType(pub(crate) u64);
impl VertexPointerType {
    pub const GL_INT: Self = Self(0x1404);
    pub const GL_DOUBLE: Self = Self(0x140a);
    pub const GL_FLOAT: Self = Self(0x1406);
    pub const GL_SHORT: Self = Self(0x1402);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DrawBufferModeATI(pub(crate) u64);
impl DrawBufferModeATI {
    pub const GL_COLOR_ATTACHMENT6_NV: Self = Self(0x8ce6);
    pub const GL_COLOR_ATTACHMENT9_NV: Self = Self(0x8ce9);
    pub const GL_COLOR_ATTACHMENT4_NV: Self = Self(0x8ce4);
    pub const GL_COLOR_ATTACHMENT5_NV: Self = Self(0x8ce5);
    pub const GL_COLOR_ATTACHMENT11_NV: Self = Self(0x8ceb);
    pub const GL_COLOR_ATTACHMENT14_NV: Self = Self(0x8cee);
    pub const GL_COLOR_ATTACHMENT15_NV: Self = Self(0x8cef);
    pub const GL_COLOR_ATTACHMENT1_NV: Self = Self(0x8ce1);
    pub const GL_COLOR_ATTACHMENT3_NV: Self = Self(0x8ce3);
    pub const GL_COLOR_ATTACHMENT10_NV: Self = Self(0x8cea);
    pub const GL_COLOR_ATTACHMENT0_NV: Self = Self(0x8ce0);
    pub const GL_COLOR_ATTACHMENT12_NV: Self = Self(0x8cec);
    pub const GL_COLOR_ATTACHMENT13_NV: Self = Self(0x8ced);
    pub const GL_COLOR_ATTACHMENT7_NV: Self = Self(0x8ce7);
    pub const GL_COLOR_ATTACHMENT2_NV: Self = Self(0x8ce2);
    pub const GL_COLOR_ATTACHMENT8_NV: Self = Self(0x8ce8);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ClipControlDepth(pub(crate) u64);
impl ClipControlDepth {
    pub const GL_NEGATIVE_ONE_TO_ONE: Self = Self(0x935e);
    pub const GL_ZERO_TO_ONE: Self = Self(0x935f);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ShadingModel(pub(crate) u64);
impl ShadingModel {
    pub const GL_FLAT: Self = Self(0x1d00);
    pub const GL_SMOOTH: Self = Self(0x1d01);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NormalPointerType(pub(crate) u64);
impl NormalPointerType {
    pub const GL_DOUBLE: Self = Self(0x140a);
    pub const GL_INT: Self = Self(0x1404);
    pub const GL_SHORT: Self = Self(0x1402);
    pub const GL_BYTE: Self = Self(0x1400);
    pub const GL_FLOAT: Self = Self(0x1406);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct InvalidateFramebufferAttachment(pub(crate) u64);
impl InvalidateFramebufferAttachment {
    pub const GL_COLOR_ATTACHMENT23: Self = Self(0x8cf7);
    pub const GL_COLOR_ATTACHMENT8_NV: Self = Self(0x8ce8);
    pub const GL_COLOR_ATTACHMENT8_EXT: Self = Self(0x8ce8);
    pub const GL_COLOR_ATTACHMENT1_NV: Self = Self(0x8ce1);
    pub const GL_COLOR_ATTACHMENT22: Self = Self(0x8cf6);
    pub const GL_COLOR_ATTACHMENT15_NV: Self = Self(0x8cef);
    pub const GL_COLOR_ATTACHMENT9_NV: Self = Self(0x8ce9);
    pub const GL_COLOR_ATTACHMENT12_EXT: Self = Self(0x8cec);
    pub const GL_COLOR_ATTACHMENT25: Self = Self(0x8cf9);
    pub const GL_COLOR_ATTACHMENT1_EXT: Self = Self(0x8ce1);
    pub const GL_COLOR_ATTACHMENT2_NV: Self = Self(0x8ce2);
    pub const GL_COLOR_ATTACHMENT21: Self = Self(0x8cf5);
    pub const GL_COLOR_ATTACHMENT29: Self = Self(0x8cfd);
    pub const GL_STENCIL_ATTACHMENT_EXT: Self = Self(0x8d20);
    pub const GL_COLOR_ATTACHMENT26: Self = Self(0x8cfa);
    pub const GL_COLOR_ATTACHMENT6: Self = Self(0x8ce6);
    pub const GL_COLOR_ATTACHMENT5: Self = Self(0x8ce5);
    pub const GL_COLOR_ATTACHMENT10_NV: Self = Self(0x8cea);
    pub const GL_COLOR_ATTACHMENT5_EXT: Self = Self(0x8ce5);
    pub const GL_COLOR_ATTACHMENT3_NV: Self = Self(0x8ce3);
    pub const GL_COLOR_ATTACHMENT4_EXT: Self = Self(0x8ce4);
    pub const GL_COLOR_ATTACHMENT0_NV: Self = Self(0x8ce0);
    pub const GL_COLOR_ATTACHMENT11_EXT: Self = Self(0x8ceb);
    pub const GL_COLOR_ATTACHMENT17: Self = Self(0x8cf1);
    pub const GL_COLOR_ATTACHMENT0: Self = Self(0x8ce0);
    pub const GL_COLOR_ATTACHMENT31: Self = Self(0x8cff);
    pub const GL_COLOR_ATTACHMENT19: Self = Self(0x8cf3);
    pub const GL_STENCIL_ATTACHMENT_OES: Self = Self(0x8d20);
    pub const GL_COLOR_ATTACHMENT4_NV: Self = Self(0x8ce4);
    pub const GL_COLOR_ATTACHMENT5_NV: Self = Self(0x8ce5);
    pub const GL_COLOR_ATTACHMENT7_NV: Self = Self(0x8ce7);
    pub const GL_COLOR_ATTACHMENT0_EXT: Self = Self(0x8ce0);
    pub const GL_COLOR_ATTACHMENT11: Self = Self(0x8ceb);
    pub const GL_COLOR_ATTACHMENT15_EXT: Self = Self(0x8cef);
    pub const GL_COLOR_ATTACHMENT0_OES: Self = Self(0x8ce0);
    pub const GL_COLOR_ATTACHMENT14: Self = Self(0x8cee);
    pub const GL_DEPTH_STENCIL_ATTACHMENT: Self = Self(0x821a);
    pub const GL_COLOR_ATTACHMENT24: Self = Self(0x8cf8);
    pub const GL_COLOR_ATTACHMENT3_EXT: Self = Self(0x8ce3);
    pub const GL_COLOR_ATTACHMENT4: Self = Self(0x8ce4);
    pub const GL_COLOR_ATTACHMENT9: Self = Self(0x8ce9);
    pub const GL_COLOR_ATTACHMENT13: Self = Self(0x8ced);
    pub const GL_COLOR_ATTACHMENT15: Self = Self(0x8cef);
    pub const GL_COLOR_ATTACHMENT2: Self = Self(0x8ce2);
    pub const GL_COLOR_ATTACHMENT20: Self = Self(0x8cf4);
    pub const GL_COLOR_ATTACHMENT18: Self = Self(0x8cf2);
    pub const GL_COLOR_ATTACHMENT6_EXT: Self = Self(0x8ce6);
    pub const GL_COLOR_ATTACHMENT28: Self = Self(0x8cfc);
    pub const GL_COLOR_ATTACHMENT16: Self = Self(0x8cf0);
    pub const GL_COLOR_ATTACHMENT14_NV: Self = Self(0x8cee);
    pub const GL_COLOR_ATTACHMENT7: Self = Self(0x8ce7);
    pub const GL_COLOR_ATTACHMENT3: Self = Self(0x8ce3);
    pub const GL_COLOR_ATTACHMENT7_EXT: Self = Self(0x8ce7);
    pub const GL_COLOR_ATTACHMENT8: Self = Self(0x8ce8);
    pub const GL_DEPTH: Self = Self(0x1801);
    pub const GL_COLOR_ATTACHMENT10: Self = Self(0x8cea);
    pub const GL_COLOR_ATTACHMENT27: Self = Self(0x8cfb);
    pub const GL_COLOR_ATTACHMENT2_EXT: Self = Self(0x8ce2);
    pub const GL_DEPTH_ATTACHMENT_OES: Self = Self(0x8d00);
    pub const GL_COLOR_ATTACHMENT13_EXT: Self = Self(0x8ced);
    pub const GL_COLOR_ATTACHMENT13_NV: Self = Self(0x8ced);
    pub const GL_COLOR_ATTACHMENT10_EXT: Self = Self(0x8cea);
    pub const GL_DEPTH_ATTACHMENT: Self = Self(0x8d00);
    pub const GL_COLOR_ATTACHMENT9_EXT: Self = Self(0x8ce9);
    pub const GL_COLOR_ATTACHMENT6_NV: Self = Self(0x8ce6);
    pub const GL_COLOR_ATTACHMENT11_NV: Self = Self(0x8ceb);
    pub const GL_COLOR: Self = Self(0x1800);
    pub const GL_COLOR_ATTACHMENT1: Self = Self(0x8ce1);
    pub const GL_COLOR_ATTACHMENT14_EXT: Self = Self(0x8cee);
    pub const GL_STENCIL: Self = Self(0x1802);
    pub const GL_DEPTH_ATTACHMENT_EXT: Self = Self(0x8d00);
    pub const GL_COLOR_ATTACHMENT12_NV: Self = Self(0x8cec);
    pub const GL_COLOR_ATTACHMENT12: Self = Self(0x8cec);
    pub const GL_COLOR_ATTACHMENT30: Self = Self(0x8cfe);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MaterialFace(pub(crate) u64);
impl MaterialFace {
    pub const GL_FRONT_AND_BACK: Self = Self(0x0408);
    pub const GL_FRONT: Self = Self(0x0404);
    pub const GL_BACK: Self = Self(0x0405);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FramebufferAttachment(pub(crate) u64);
impl FramebufferAttachment {
    pub const GL_COLOR_ATTACHMENT9: Self = Self(0x8ce9);
    pub const GL_STENCIL_ATTACHMENT: Self = Self(0x8d20);
    pub const GL_COLOR_ATTACHMENT14: Self = Self(0x8cee);
    pub const GL_COLOR_ATTACHMENT18: Self = Self(0x8cf2);
    pub const GL_COLOR_ATTACHMENT30: Self = Self(0x8cfe);
    pub const GL_COLOR_ATTACHMENT7: Self = Self(0x8ce7);
    pub const GL_COLOR_ATTACHMENT12: Self = Self(0x8cec);
    pub const GL_COLOR_ATTACHMENT0: Self = Self(0x8ce0);
    pub const GL_COLOR_ATTACHMENT17: Self = Self(0x8cf1);
    pub const GL_COLOR_ATTACHMENT24: Self = Self(0x8cf8);
    pub const GL_COLOR_ATTACHMENT4: Self = Self(0x8ce4);
    pub const GL_COLOR_ATTACHMENT20: Self = Self(0x8cf4);
    pub const GL_COLOR_ATTACHMENT28: Self = Self(0x8cfc);
    pub const GL_COLOR_ATTACHMENT8: Self = Self(0x8ce8);
    pub const GL_COLOR_ATTACHMENT11: Self = Self(0x8ceb);
    pub const GL_DEPTH_ATTACHMENT: Self = Self(0x8d00);
    pub const GL_COLOR_ATTACHMENT19: Self = Self(0x8cf3);
    pub const GL_COLOR_ATTACHMENT26: Self = Self(0x8cfa);
    pub const GL_COLOR_ATTACHMENT23: Self = Self(0x8cf7);
    pub const GL_COLOR_ATTACHMENT16: Self = Self(0x8cf0);
    pub const GL_COLOR_ATTACHMENT10: Self = Self(0x8cea);
    pub const GL_COLOR_ATTACHMENT6: Self = Self(0x8ce6);
    pub const GL_COLOR_ATTACHMENT1: Self = Self(0x8ce1);
    pub const GL_COLOR_ATTACHMENT13: Self = Self(0x8ced);
    pub const GL_COLOR_ATTACHMENT25: Self = Self(0x8cf9);
    pub const GL_COLOR_ATTACHMENT27: Self = Self(0x8cfb);
    pub const GL_COLOR_ATTACHMENT29: Self = Self(0x8cfd);
    pub const GL_COLOR_ATTACHMENT3: Self = Self(0x8ce3);
    pub const GL_COLOR_ATTACHMENT5: Self = Self(0x8ce5);
    pub const GL_COLOR_ATTACHMENT22: Self = Self(0x8cf6);
    pub const GL_COLOR_ATTACHMENT31: Self = Self(0x8cff);
    pub const GL_DEPTH_STENCIL_ATTACHMENT: Self = Self(0x821a);
    pub const GL_COLOR_ATTACHMENT15: Self = Self(0x8cef);
    pub const GL_COLOR_ATTACHMENT21: Self = Self(0x8cf5);
    pub const GL_COLOR_ATTACHMENT2: Self = Self(0x8ce2);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CombinerBiasNV(pub(crate) u64);
impl CombinerBiasNV {
    pub const GL_NONE: Self = Self(0);
    pub const GL_BIAS_BY_NEGATIVE_ONE_HALF_NV: Self = Self(0x8541);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ScalarType(pub(crate) u64);
impl ScalarType {
    pub const GL_UNSIGNED_SHORT: Self = Self(0x1403);
    pub const GL_UNSIGNED_INT: Self = Self(0x1405);
    pub const GL_UNSIGNED_BYTE: Self = Self(0x1401);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ArrayObjectUsageATI(pub(crate) u64);
impl ArrayObjectUsageATI {
    pub const GL_DYNAMIC_ATI: Self = Self(0x8761);
    pub const GL_STATIC_ATI: Self = Self(0x8760);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DrawBufferMode(pub(crate) u64);
impl DrawBufferMode {
    pub const GL_COLOR_ATTACHMENT8: Self = Self(0x8ce8);
    pub const GL_BACK_RIGHT: Self = Self(0x0403);
    pub const GL_COLOR_ATTACHMENT23: Self = Self(0x8cf7);
    pub const GL_AUX3: Self = Self(0x040c);
    pub const GL_COLOR_ATTACHMENT14: Self = Self(0x8cee);
    pub const GL_COLOR_ATTACHMENT13: Self = Self(0x8ced);
    pub const GL_COLOR_ATTACHMENT0: Self = Self(0x8ce0);
    pub const GL_COLOR_ATTACHMENT11: Self = Self(0x8ceb);
    pub const GL_COLOR_ATTACHMENT16: Self = Self(0x8cf0);
    pub const GL_COLOR_ATTACHMENT1: Self = Self(0x8ce1);
    pub const GL_COLOR_ATTACHMENT20: Self = Self(0x8cf4);
    pub const GL_LEFT: Self = Self(0x0406);
    pub const GL_COLOR_ATTACHMENT2: Self = Self(0x8ce2);
    pub const GL_COLOR_ATTACHMENT26: Self = Self(0x8cfa);
    pub const GL_COLOR_ATTACHMENT28: Self = Self(0x8cfc);
    pub const GL_AUX2: Self = Self(0x040b);
    pub const GL_NONE: Self = Self(0);
    pub const GL_COLOR_ATTACHMENT6: Self = Self(0x8ce6);
    pub const GL_COLOR_ATTACHMENT22: Self = Self(0x8cf6);
    pub const GL_COLOR_ATTACHMENT4: Self = Self(0x8ce4);
    pub const GL_BACK: Self = Self(0x0405);
    pub const GL_COLOR_ATTACHMENT3: Self = Self(0x8ce3);
    pub const GL_COLOR_ATTACHMENT10: Self = Self(0x8cea);
    pub const GL_NONE_OES: Self = Self(0);
    pub const GL_RIGHT: Self = Self(0x0407);
    pub const GL_FRONT_AND_BACK: Self = Self(0x0408);
    pub const GL_COLOR_ATTACHMENT18: Self = Self(0x8cf2);
    pub const GL_COLOR_ATTACHMENT12: Self = Self(0x8cec);
    pub const GL_COLOR_ATTACHMENT30: Self = Self(0x8cfe);
    pub const GL_COLOR_ATTACHMENT31: Self = Self(0x8cff);
    pub const GL_FRONT_RIGHT: Self = Self(0x0401);
    pub const GL_COLOR_ATTACHMENT7: Self = Self(0x8ce7);
    pub const GL_FRONT_LEFT: Self = Self(0x0400);
    pub const GL_BACK_LEFT: Self = Self(0x0402);
    pub const GL_COLOR_ATTACHMENT27: Self = Self(0x8cfb);
    pub const GL_COLOR_ATTACHMENT24: Self = Self(0x8cf8);
    pub const GL_COLOR_ATTACHMENT29: Self = Self(0x8cfd);
    pub const GL_COLOR_ATTACHMENT21: Self = Self(0x8cf5);
    pub const GL_COLOR_ATTACHMENT19: Self = Self(0x8cf3);
    pub const GL_AUX0: Self = Self(0x0409);
    pub const GL_AUX1: Self = Self(0x040a);
    pub const GL_COLOR_ATTACHMENT9: Self = Self(0x8ce9);
    pub const GL_FRONT: Self = Self(0x0404);
    pub const GL_COLOR_ATTACHMENT25: Self = Self(0x8cf9);
    pub const GL_COLOR_ATTACHMENT15: Self = Self(0x8cef);
    pub const GL_COLOR_ATTACHMENT5: Self = Self(0x8ce5);
    pub const GL_COLOR_ATTACHMENT17: Self = Self(0x8cf1);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct HintTarget(pub(crate) u64);
impl HintTarget {
    pub const GL_BINNING_CONTROL_HINT_QCOM: Self = Self(0x8fb0);
    pub const GL_POLYGON_SMOOTH_HINT: Self = Self(0x0c53);
    pub const GL_GENERATE_MIPMAP_HINT_SGIS: Self = Self(0x8192);
    pub const GL_NATIVE_GRAPHICS_BEGIN_HINT_PGI: Self = Self(0x1_a203);
    pub const GL_ALLOW_DRAW_WIN_HINT_PGI: Self = Self(0x1_a20f);
    pub const GL_ALLOW_DRAW_FRG_HINT_PGI: Self = Self(0x1_a210);
    pub const GL_STRICT_SCISSOR_HINT_PGI: Self = Self(0x1_a218);
    pub const GL_CLIP_NEAR_HINT_PGI: Self = Self(0x1_a220);
    pub const GL_CONSERVE_MEMORY_HINT_PGI: Self = Self(0x1_a1fd);
    pub const GL_RECLAIM_MEMORY_HINT_PGI: Self = Self(0x1_a1fe);
    pub const GL_NATIVE_GRAPHICS_END_HINT_PGI: Self = Self(0x1_a204);
    pub const GL_ALLOW_DRAW_MEM_HINT_PGI: Self = Self(0x1_a211);
    pub const GL_VERTEX_ARRAY_STORAGE_HINT_APPLE: Self = Self(0x851f);
    pub const GL_STRICT_LIGHTING_HINT_PGI: Self = Self(0x1_a217);
    pub const GL_VERTEX_CONSISTENT_HINT_PGI: Self = Self(0x1_a22b);
    pub const GL_FULL_STIPPLE_HINT_PGI: Self = Self(0x1_a219);
    pub const GL_TRANSFORM_HINT_APPLE: Self = Self(0x85b1);
    pub const GL_MULTISAMPLE_FILTER_HINT_NV: Self = Self(0x8534);
    pub const GL_TEXTURE_MULTI_BUFFER_HINT_SGIX: Self = Self(0x812e);
    pub const GL_PROGRAM_BINARY_RETRIEVABLE_HINT: Self = Self(0x8257);
    pub const GL_LINE_QUALITY_HINT_SGIX: Self = Self(0x835b);
    pub const GL_FRAGMENT_SHADER_DERIVATIVE_HINT_OES: Self = Self(0x8b8b);
    pub const GL_PACK_CMYK_HINT_EXT: Self = Self(0x800e);
    pub const GL_BACK_NORMALS_HINT_PGI: Self = Self(0x1_a223);
    pub const GL_VERTEX_DATA_HINT_PGI: Self = Self(0x1_a22a);
    pub const GL_WIDE_LINE_HINT_PGI: Self = Self(0x1_a222);
    pub const GL_TEXTURE_COMPRESSION_HINT: Self = Self(0x84ef);
    pub const GL_POINT_SMOOTH_HINT: Self = Self(0x0c51);
    pub const GL_CONVOLUTION_HINT_SGIX: Self = Self(0x8316);
    pub const GL_MATERIAL_SIDE_HINT_PGI: Self = Self(0x1_a22c);
    pub const GL_FRAGMENT_SHADER_DERIVATIVE_HINT: Self = Self(0x8b8b);
    pub const GL_VERTEX_PRECLIP_SGIX: Self = Self(0x83ee);
    pub const GL_TEXTURE_COMPRESSION_HINT_ARB: Self = Self(0x84ef);
    pub const GL_STRICT_DEPTHFUNC_HINT_PGI: Self = Self(0x1_a216);
    pub const GL_UNPACK_CMYK_HINT_EXT: Self = Self(0x800f);
    pub const GL_MAX_VERTEX_HINT_PGI: Self = Self(0x1_a22d);
    pub const GL_CLIP_FAR_HINT_PGI: Self = Self(0x1_a221);
    pub const GL_FRAGMENT_SHADER_DERIVATIVE_HINT_ARB: Self = Self(0x8b8b);
    pub const GL_CLIP_VOLUME_CLIPPING_HINT_EXT: Self = Self(0x80f0);
    pub const GL_GENERATE_MIPMAP_HINT: Self = Self(0x8192);
    pub const GL_LINE_SMOOTH_HINT: Self = Self(0x0c52);
    pub const GL_PHONG_HINT_WIN: Self = Self(0x80eb);
    pub const GL_ALWAYS_SOFT_HINT_PGI: Self = Self(0x1_a20d);
    pub const GL_ALWAYS_FAST_HINT_PGI: Self = Self(0x1_a20c);
    pub const GL_ALLOW_DRAW_OBJ_HINT_PGI: Self = Self(0x1_a20e);
    pub const GL_VERTEX_PRECLIP_HINT_SGIX: Self = Self(0x83ef);
    pub const GL_TEXTURE_STORAGE_HINT_APPLE: Self = Self(0x85bc);
    pub const GL_FOG_HINT: Self = Self(0x0c54);
    pub const GL_PERSPECTIVE_CORRECTION_HINT: Self = Self(0x0c50);
    pub const GL_SCALEBIAS_HINT_SGIX: Self = Self(0x8322);
    pub const GL_PREFER_DOUBLEBUFFER_HINT_PGI: Self = Self(0x1_a1f8);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SubroutineParameterName(pub(crate) u64);
impl SubroutineParameterName {
    pub const GL_COMPATIBLE_SUBROUTINES: Self = Self(0x8e4b);
    pub const GL_UNIFORM_NAME_LENGTH: Self = Self(0x8a39);
    pub const GL_UNIFORM_SIZE: Self = Self(0x8a38);
    pub const GL_NUM_COMPATIBLE_SUBROUTINES: Self = Self(0x8e4a);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ProgramResourceProperty(pub(crate) u64);
impl ProgramResourceProperty {
    pub const GL_BUFFER_BINDING: Self = Self(0x9302);
    pub const GL_REFERENCED_BY_FRAGMENT_SHADER: Self = Self(0x930a);
    pub const GL_OFFSET: Self = Self(0x92fc);
    pub const GL_TYPE: Self = Self(0x92fa);
    pub const GL_IS_ROW_MAJOR: Self = Self(0x9300);
    pub const GL_NUM_ACTIVE_VARIABLES: Self = Self(0x9304);
    pub const GL_COMPATIBLE_SUBROUTINES: Self = Self(0x8e4b);
    pub const GL_REFERENCED_BY_COMPUTE_SHADER: Self = Self(0x930b);
    pub const GL_ATOMIC_COUNTER_BUFFER_INDEX: Self = Self(0x9301);
    pub const GL_REFERENCED_BY_TESS_CONTROL_SHADER: Self = Self(0x9307);
    pub const GL_NUM_COMPATIBLE_SUBROUTINES: Self = Self(0x8e4a);
    pub const GL_IS_PER_PATCH: Self = Self(0x92e7);
    pub const GL_REFERENCED_BY_TESS_EVALUATION_SHADER: Self = Self(0x9308);
    pub const GL_ARRAY_STRIDE: Self = Self(0x92fe);
    pub const GL_ACTIVE_VARIABLES: Self = Self(0x9305);
    pub const GL_LOCATION_INDEX: Self = Self(0x930f);
    pub const GL_REFERENCED_BY_VERTEX_SHADER: Self = Self(0x9306);
    pub const GL_LOCATION_COMPONENT: Self = Self(0x934a);
    pub const GL_TRANSFORM_FEEDBACK_BUFFER_STRIDE: Self = Self(0x934c);
    pub const GL_BLOCK_INDEX: Self = Self(0x92fd);
    pub const GL_UNIFORM: Self = Self(0x92e1);
    pub const GL_TOP_LEVEL_ARRAY_SIZE: Self = Self(0x930c);
    pub const GL_LOCATION: Self = Self(0x930e);
    pub const GL_NAME_LENGTH: Self = Self(0x92f9);
    pub const GL_REFERENCED_BY_GEOMETRY_SHADER: Self = Self(0x9309);
    pub const GL_TOP_LEVEL_ARRAY_STRIDE: Self = Self(0x930d);
    pub const GL_BUFFER_DATA_SIZE: Self = Self(0x9303);
    pub const GL_MATRIX_STRIDE: Self = Self(0x92ff);
    pub const GL_TRANSFORM_FEEDBACK_BUFFER_INDEX: Self = Self(0x934b);
    pub const GL_ARRAY_SIZE: Self = Self(0x92fb);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MeshMode1(pub(crate) u64);
impl MeshMode1 {
    pub const GL_POINT: Self = Self(0x1b00);
    pub const GL_LINE: Self = Self(0x1b01);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BufferPointerNameARB(pub(crate) u64);
impl BufferPointerNameARB {
    pub const GL_BUFFER_MAP_POINTER_ARB: Self = Self(0x88bd);
    pub const GL_BUFFER_MAP_POINTER: Self = Self(0x88bd);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TransformFeedbackPName(pub(crate) u64);
impl TransformFeedbackPName {
    pub const GL_TRANSFORM_FEEDBACK_BUFFER_START: Self = Self(0x8c84);
    pub const GL_TRANSFORM_FEEDBACK_PAUSED: Self = Self(0x8e23);
    pub const GL_TRANSFORM_FEEDBACK_BUFFER_BINDING: Self = Self(0x8c8f);
    pub const GL_TRANSFORM_FEEDBACK_BUFFER_SIZE: Self = Self(0x8c85);
    pub const GL_TRANSFORM_FEEDBACK_ACTIVE: Self = Self(0x8e24);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PathFontTarget(pub(crate) u64);
impl PathFontTarget {
    pub const GL_STANDARD_FONT_NAME_NV: Self = Self(0x9072);
    pub const GL_SYSTEM_FONT_NAME_NV: Self = Self(0x9073);
    pub const GL_FILE_NAME_NV: Self = Self(0x9074);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FrontFaceDirection(pub(crate) u64);
impl FrontFaceDirection {
    pub const GL_CCW: Self = Self(0x0901);
    pub const GL_CW: Self = Self(0x0900);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Buffer(pub(crate) u64);
impl Buffer {
    pub const GL_COLOR: Self = Self(0x1800);
    pub const GL_DEPTH: Self = Self(0x1801);
    pub const GL_STENCIL: Self = Self(0x1802);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EvalMapsModeNV(pub(crate) u64);
impl EvalMapsModeNV {
    pub const GL_FILL_NV: Self = Self(0x1b02);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RegisterCombinerPname(pub(crate) u64);
impl RegisterCombinerPname {
    pub const GL_PRIMARY_COLOR_EXT: Self = Self(0x8577);
    pub const GL_SRC1_ALPHA: Self = Self(0x8589);
    pub const GL_SOURCE1_RGB_ARB: Self = Self(0x8581);
    pub const GL_SOURCE2_RGB_EXT: Self = Self(0x8582);
    pub const GL_OPERAND0_RGB_ARB: Self = Self(0x8590);
    pub const GL_OPERAND2_RGB_ARB: Self = Self(0x8592);
    pub const GL_OPERAND0_ALPHA_EXT: Self = Self(0x8598);
    pub const GL_COMBINE_ARB: Self = Self(0x8570);
    pub const GL_OPERAND2_RGB: Self = Self(0x8592);
    pub const GL_OPERAND2_ALPHA_EXT: Self = Self(0x859a);
    pub const GL_SOURCE1_ALPHA_ARB: Self = Self(0x8589);
    pub const GL_COMBINE_ALPHA: Self = Self(0x8572);
    pub const GL_COMBINE_RGB_ARB: Self = Self(0x8571);
    pub const GL_SOURCE2_RGB: Self = Self(0x8582);
    pub const GL_OPERAND0_RGB_EXT: Self = Self(0x8590);
    pub const GL_OPERAND2_RGB_EXT: Self = Self(0x8592);
    pub const GL_ADD_SIGNED: Self = Self(0x8574);
    pub const GL_SOURCE1_RGB_EXT: Self = Self(0x8581);
    pub const GL_SRC2_RGB: Self = Self(0x8582);
    pub const GL_SOURCE0_ALPHA_ARB: Self = Self(0x8588);
    pub const GL_RGB_SCALE_ARB: Self = Self(0x8573);
    pub const GL_SRC1_ALPHA_EXT: Self = Self(0x8589);
    pub const GL_OPERAND0_RGB: Self = Self(0x8590);
    pub const GL_OPERAND1_RGB_ARB: Self = Self(0x8591);
    pub const GL_RGB_SCALE: Self = Self(0x8573);
    pub const GL_SOURCE3_RGB_NV: Self = Self(0x8583);
    pub const GL_SOURCE0_ALPHA_EXT: Self = Self(0x8588);
    pub const GL_SRC0_RGB: Self = Self(0x8580);
    pub const GL_PRIMARY_COLOR: Self = Self(0x8577);
    pub const GL_OPERAND1_ALPHA_ARB: Self = Self(0x8599);
    pub const GL_SOURCE1_ALPHA_EXT: Self = Self(0x8589);
    pub const GL_COMBINE_ALPHA_ARB: Self = Self(0x8572);
    pub const GL_CONSTANT_EXT: Self = Self(0x8576);
    pub const GL_INTERPOLATE_ARB: Self = Self(0x8575);
    pub const GL_SOURCE0_RGB_ARB: Self = Self(0x8580);
    pub const GL_SOURCE0_ALPHA: Self = Self(0x8588);
    pub const GL_OPERAND1_ALPHA_EXT: Self = Self(0x8599);
    pub const GL_OPERAND3_ALPHA_NV: Self = Self(0x859b);
    pub const GL_SOURCE0_RGB_EXT: Self = Self(0x8580);
    pub const GL_INTERPOLATE: Self = Self(0x8575);
    pub const GL_SOURCE2_ALPHA: Self = Self(0x858a);
    pub const GL_SOURCE2_ALPHA_ARB: Self = Self(0x858a);
    pub const GL_OPERAND1_ALPHA: Self = Self(0x8599);
    pub const GL_COMBINE_ALPHA_EXT: Self = Self(0x8572);
    pub const GL_OPERAND1_RGB: Self = Self(0x8591);
    pub const GL_SOURCE2_RGB_ARB: Self = Self(0x8582);
    pub const GL_OPERAND2_ALPHA_ARB: Self = Self(0x859a);
    pub const GL_CONSTANT: Self = Self(0x8576);
    pub const GL_INTERPOLATE_EXT: Self = Self(0x8575);
    pub const GL_SOURCE2_ALPHA_EXT: Self = Self(0x858a);
    pub const GL_SRC2_ALPHA: Self = Self(0x858a);
    pub const GL_SOURCE3_ALPHA_NV: Self = Self(0x858b);
    pub const GL_CONSTANT_ARB: Self = Self(0x8576);
    pub const GL_SOURCE0_RGB: Self = Self(0x8580);
    pub const GL_COMBINE_RGB_EXT: Self = Self(0x8571);
    pub const GL_OPERAND2_ALPHA: Self = Self(0x859a);
    pub const GL_PREVIOUS_ARB: Self = Self(0x8578);
    pub const GL_SRC1_RGB: Self = Self(0x8581);
    pub const GL_SOURCE1_RGB: Self = Self(0x8581);
    pub const GL_SOURCE1_ALPHA: Self = Self(0x8589);
    pub const GL_COMBINE: Self = Self(0x8570);
    pub const GL_PRIMARY_COLOR_ARB: Self = Self(0x8577);
    pub const GL_PREVIOUS: Self = Self(0x8578);
    pub const GL_COMBINE_RGB: Self = Self(0x8571);
    pub const GL_RGB_SCALE_EXT: Self = Self(0x8573);
    pub const GL_PREVIOUS_EXT: Self = Self(0x8578);
    pub const GL_OPERAND0_ALPHA_ARB: Self = Self(0x8598);
    pub const GL_OPERAND1_RGB_EXT: Self = Self(0x8591);
    pub const GL_SRC0_ALPHA: Self = Self(0x8588);
    pub const GL_ADD_SIGNED_EXT: Self = Self(0x8574);
    pub const GL_CONSTANT_NV: Self = Self(0x8576);
    pub const GL_ADD_SIGNED_ARB: Self = Self(0x8574);
    pub const GL_COMBINE_EXT: Self = Self(0x8570);
    pub const GL_OPERAND3_RGB_NV: Self = Self(0x8593);
    pub const GL_OPERAND0_ALPHA: Self = Self(0x8598);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CullParameterEXT(pub(crate) u64);
impl CullParameterEXT {
    pub const GL_CULL_VERTEX_EYE_POSITION_EXT: Self = Self(0x81ab);
    pub const GL_CULL_VERTEX_OBJECT_POSITION_EXT: Self = Self(0x81ac);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ColorMaterialFace(pub(crate) u64);
impl ColorMaterialFace {
    pub const GL_FRONT: Self = Self(0x0404);
    pub const GL_FRONT_AND_BACK: Self = Self(0x0408);
    pub const GL_BACK: Self = Self(0x0405);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ErrorCode(pub(crate) u64);
impl ErrorCode {
    pub const GL_INVALID_FRAMEBUFFER_OPERATION_OES: Self = Self(0x0506);
    pub const GL_STACK_OVERFLOW: Self = Self(0x0503);
    pub const GL_TABLE_TOO_LARGE: Self = Self(0x8031);
    pub const GL_NO_ERROR: Self = Self(0);
    pub const GL_OUT_OF_MEMORY: Self = Self(0x0505);
    pub const GL_TABLE_TOO_LARGE_EXT: Self = Self(0x8031);
    pub const GL_INVALID_FRAMEBUFFER_OPERATION_EXT: Self = Self(0x0506);
    pub const GL_INVALID_ENUM: Self = Self(0x0500);
    pub const GL_INVALID_VALUE: Self = Self(0x0501);
    pub const GL_INVALID_OPERATION: Self = Self(0x0502);
    pub const GL_STACK_UNDERFLOW: Self = Self(0x0504);
    pub const GL_INVALID_FRAMEBUFFER_OPERATION: Self = Self(0x0506);
    pub const GL_TEXTURE_TOO_LARGE_EXT: Self = Self(0x8065);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EvalTargetNV(pub(crate) u64);
impl EvalTargetNV {
    pub const GL_EVAL_TRIANGULAR_2D_NV: Self = Self(0x86c1);
    pub const GL_EVAL_2D_NV: Self = Self(0x86c0);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct HintMode(pub(crate) u64);
impl HintMode {
    pub const GL_DONT_CARE: Self = Self(0x1100);
    pub const GL_FASTEST: Self = Self(0x1101);
    pub const GL_NICEST: Self = Self(0x1102);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ObjectIdentifier(pub(crate) u64);
impl ObjectIdentifier {
    pub const GL_RENDERBUFFER: Self = Self(0x8d41);
    pub const GL_PROGRAM: Self = Self(0x82e2);
    pub const GL_SHADER: Self = Self(0x82e1);
    pub const GL_QUERY: Self = Self(0x82e3);
    pub const GL_VERTEX_ARRAY: Self = Self(0x8074);
    pub const GL_TEXTURE: Self = Self(0x1702);
    pub const GL_SAMPLER: Self = Self(0x82e6);
    pub const GL_FRAMEBUFFER: Self = Self(0x8d40);
    pub const GL_BUFFER: Self = Self(0x82e0);
    pub const GL_TRANSFORM_FEEDBACK: Self = Self(0x8e22);
    pub const GL_PROGRAM_PIPELINE: Self = Self(0x82e4);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VertexBufferObjectParameter(pub(crate) u64);
impl VertexBufferObjectParameter {
    pub const GL_BUFFER_SIZE: Self = Self(0x8764);
    pub const GL_BUFFER_USAGE: Self = Self(0x8765);
    pub const GL_BUFFER_STORAGE_FLAGS: Self = Self(0x8220);
    pub const GL_BUFFER_MAPPED: Self = Self(0x88bc);
    pub const GL_BUFFER_ACCESS: Self = Self(0x88bb);
    pub const GL_BUFFER_ACCESS_FLAGS: Self = Self(0x911f);
    pub const GL_BUFFER_MAP_OFFSET: Self = Self(0x9121);
    pub const GL_BUFFER_IMMUTABLE_STORAGE: Self = Self(0x821f);
    pub const GL_BUFFER_MAP_LENGTH: Self = Self(0x9120);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VertexAttribIType(pub(crate) u64);
impl VertexAttribIType {
    pub const GL_UNSIGNED_BYTE: Self = Self(0x1401);
    pub const GL_UNSIGNED_INT: Self = Self(0x1405);
    pub const GL_BYTE: Self = Self(0x1400);
    pub const GL_SHORT: Self = Self(0x1402);
    pub const GL_INT: Self = Self(0x1404);
    pub const GL_UNSIGNED_SHORT: Self = Self(0x1403);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BufferStorageTarget(pub(crate) u64);
impl BufferStorageTarget {
    pub const GL_TEXTURE_BUFFER: Self = Self(0x8c2a);
    pub const GL_ATOMIC_COUNTER_BUFFER: Self = Self(0x92c0);
    pub const GL_COPY_READ_BUFFER: Self = Self(0x8f36);
    pub const GL_COPY_WRITE_BUFFER: Self = Self(0x8f37);
    pub const GL_UNIFORM_BUFFER: Self = Self(0x8a11);
    pub const GL_DRAW_INDIRECT_BUFFER: Self = Self(0x8f3f);
    pub const GL_ELEMENT_ARRAY_BUFFER: Self = Self(0x8893);
    pub const GL_PIXEL_PACK_BUFFER: Self = Self(0x88eb);
    pub const GL_TRANSFORM_FEEDBACK_BUFFER: Self = Self(0x8c8e);
    pub const GL_ARRAY_BUFFER: Self = Self(0x8892);
    pub const GL_SHADER_STORAGE_BUFFER: Self = Self(0x90d2);
    pub const GL_PIXEL_UNPACK_BUFFER: Self = Self(0x88ec);
    pub const GL_QUERY_BUFFER: Self = Self(0x9192);
    pub const GL_DISPATCH_INDIRECT_BUFFER: Self = Self(0x90ee);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SyncParameterName(pub(crate) u64);
impl SyncParameterName {
    pub const GL_OBJECT_TYPE: Self = Self(0x9112);
    pub const GL_SYNC_FLAGS: Self = Self(0x9115);
    pub const GL_SYNC_CONDITION: Self = Self(0x9113);
    pub const GL_SYNC_STATUS: Self = Self(0x9114);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BlendEquationModeEXT(pub(crate) u64);
impl BlendEquationModeEXT {
    pub const GL_FUNC_ADD: Self = Self(0x8006);
    pub const GL_FUNC_SUBTRACT_EXT: Self = Self(0x800a);
    pub const GL_MIN_EXT: Self = Self(0x8007);
    pub const GL_ALPHA_MIN_SGIX: Self = Self(0x8320);
    pub const GL_FUNC_REVERSE_SUBTRACT: Self = Self(0x800b);
    pub const GL_MAX: Self = Self(0x8008);
    pub const GL_ALPHA_MAX_SGIX: Self = Self(0x8321);
    pub const GL_FUNC_REVERSE_SUBTRACT_EXT: Self = Self(0x800b);
    pub const GL_FUNC_ADD_EXT: Self = Self(0x8006);
    pub const GL_MIN: Self = Self(0x8007);
    pub const GL_MAX_EXT: Self = Self(0x8008);
    pub const GL_FUNC_SUBTRACT: Self = Self(0x800a);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VertexAttribPointerPropertyARB(pub(crate) u64);
impl VertexAttribPointerPropertyARB {
    pub const GL_VERTEX_ATTRIB_ARRAY_POINTER_ARB: Self = Self(0x8645);
    pub const GL_VERTEX_ATTRIB_ARRAY_POINTER: Self = Self(0x8645);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DebugSource(pub(crate) u64);
impl DebugSource {
    pub const GL_DEBUG_SOURCE_OTHER: Self = Self(0x824b);
    pub const GL_DEBUG_SOURCE_WINDOW_SYSTEM: Self = Self(0x8247);
    pub const GL_DEBUG_SOURCE_API: Self = Self(0x8246);
    pub const GL_DEBUG_SOURCE_SHADER_COMPILER: Self = Self(0x8248);
    pub const GL_DONT_CARE: Self = Self(0x1100);
    pub const GL_DEBUG_SOURCE_THIRD_PARTY: Self = Self(0x8249);
    pub const GL_DEBUG_SOURCE_APPLICATION: Self = Self(0x824a);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ConditionalRenderMode(pub(crate) u64);
impl ConditionalRenderMode {
    pub const GL_QUERY_BY_REGION_NO_WAIT: Self = Self(0x8e16);
    pub const GL_QUERY_WAIT_INVERTED: Self = Self(0x8e17);
    pub const GL_QUERY_NO_WAIT: Self = Self(0x8e14);
    pub const GL_QUERY_BY_REGION_NO_WAIT_INVERTED: Self = Self(0x8e1a);
    pub const GL_QUERY_WAIT: Self = Self(0x8e13);
    pub const GL_QUERY_BY_REGION_WAIT_INVERTED: Self = Self(0x8e19);
    pub const GL_QUERY_BY_REGION_WAIT: Self = Self(0x8e15);
    pub const GL_QUERY_NO_WAIT_INVERTED: Self = Self(0x8e18);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PixelTransformTargetEXT(pub(crate) u64);
impl PixelTransformTargetEXT {
    pub const GL_PIXEL_TRANSFORM_2D_EXT: Self = Self(0x8330);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GetColorTableParameterPName(pub(crate) u64);
impl GetColorTableParameterPName {
    pub const GL_COLOR_TABLE_BIAS: Self = Self(0x80d7);
    pub const GL_COLOR_TABLE_BLUE_SIZE: Self = Self(0x80dc);
    pub const GL_COLOR_TABLE_ALPHA_SIZE: Self = Self(0x80dd);
    pub const GL_COLOR_TABLE_FORMAT: Self = Self(0x80d8);
    pub const GL_COLOR_TABLE_LUMINANCE_SIZE: Self = Self(0x80de);
    pub const GL_COLOR_TABLE_RED_SIZE: Self = Self(0x80da);
    pub const GL_COLOR_TABLE_GREEN_SIZE: Self = Self(0x80db);
    pub const GL_COLOR_TABLE_INTENSITY_SIZE: Self = Self(0x80df);
    pub const GL_COLOR_TABLE_WIDTH: Self = Self(0x80d9);
    pub const GL_COLOR_TABLE_SCALE: Self = Self(0x80d6);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FogPointerTypeIBM(pub(crate) u64);
impl FogPointerTypeIBM {
    pub const GL_FLOAT: Self = Self(0x1406);
    pub const GL_DOUBLE: Self = Self(0x140a);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct UniformType(pub(crate) u64);
impl UniformType {
    pub const GL_SAMPLER_1D: Self = Self(0x8b5d);
    pub const GL_SAMPLER_BUFFER: Self = Self(0x8dc2);
    pub const GL_BOOL_VEC2: Self = Self(0x8b57);
    pub const GL_INT_SAMPLER_CUBE_MAP_ARRAY: Self = Self(0x900e);
    pub const GL_DOUBLE: Self = Self(0x140a);
    pub const GL_FLOAT: Self = Self(0x1406);
    pub const GL_UNSIGNED_INT_VEC4: Self = Self(0x8dc8);
    pub const GL_UNSIGNED_INT_SAMPLER_2D: Self = Self(0x8dd2);
    pub const GL_UNSIGNED_INT_VEC3: Self = Self(0x8dc7);
    pub const GL_DOUBLE_MAT2x3: Self = Self(0x8f49);
    pub const GL_FLOAT_VEC2: Self = Self(0x8b50);
    pub const GL_DOUBLE_MAT4: Self = Self(0x8f48);
    pub const GL_DOUBLE_VEC4: Self = Self(0x8ffe);
    pub const GL_SAMPLER_2D_MULTISAMPLE: Self = Self(0x9108);
    pub const GL_UNSIGNED_INT: Self = Self(0x1405);
    pub const GL_INT_VEC4: Self = Self(0x8b55);
    pub const GL_SAMPLER_2D_RECT_SHADOW: Self = Self(0x8b64);
    pub const GL_FLOAT_MAT2x4: Self = Self(0x8b66);
    pub const GL_INT_SAMPLER_2D_MULTISAMPLE_ARRAY: Self = Self(0x910c);
    pub const GL_FLOAT_VEC4: Self = Self(0x8b52);
    pub const GL_UNSIGNED_INT_SAMPLER_CUBE: Self = Self(0x8dd4);
    pub const GL_UNSIGNED_INT_SAMPLER_1D: Self = Self(0x8dd1);
    pub const GL_UNSIGNED_INT_SAMPLER_2D_RECT: Self = Self(0x8dd5);
    pub const GL_UNSIGNED_INT_SAMPLER_3D: Self = Self(0x8dd3);
    pub const GL_SAMPLER_2D_ARRAY_SHADOW: Self = Self(0x8dc4);
    pub const GL_SAMPLER_CUBE: Self = Self(0x8b60);
    pub const GL_INT_VEC2: Self = Self(0x8b53);
    pub const GL_UNSIGNED_INT_SAMPLER_2D_MULTISAMPLE_ARRAY: Self = Self(0x910d);
    pub const GL_SAMPLER_1D_SHADOW: Self = Self(0x8b61);
    pub const GL_FLOAT_MAT4: Self = Self(0x8b5c);
    pub const GL_UNSIGNED_INT_SAMPLER_BUFFER: Self = Self(0x8dd8);
    pub const GL_SAMPLER_CUBE_MAP_ARRAY_SHADOW: Self = Self(0x900d);
    pub const GL_DOUBLE_MAT4x3: Self = Self(0x8f4e);
    pub const GL_SAMPLER_2D_SHADOW: Self = Self(0x8b62);
    pub const GL_INT_SAMPLER_2D: Self = Self(0x8dca);
    pub const GL_DOUBLE_MAT3x2: Self = Self(0x8f4b);
    pub const GL_INT_VEC3: Self = Self(0x8b54);
    pub const GL_SAMPLER_3D: Self = Self(0x8b5f);
    pub const GL_FLOAT_MAT3x2: Self = Self(0x8b67);
    pub const GL_UNSIGNED_INT_SAMPLER_CUBE_MAP_ARRAY: Self = Self(0x900f);
    pub const GL_SAMPLER_2D: Self = Self(0x8b5e);
    pub const GL_INT_SAMPLER_3D: Self = Self(0x8dcb);
    pub const GL_DOUBLE_VEC2: Self = Self(0x8ffc);
    pub const GL_BOOL: Self = Self(0x8b56);
    pub const GL_INT_SAMPLER_CUBE: Self = Self(0x8dcc);
    pub const GL_INT_SAMPLER_2D_RECT: Self = Self(0x8dcd);
    pub const GL_SAMPLER_2D_RECT: Self = Self(0x8b63);
    pub const GL_FLOAT_MAT2: Self = Self(0x8b5a);
    pub const GL_DOUBLE_MAT2x4: Self = Self(0x8f4a);
    pub const GL_FLOAT_MAT2x3: Self = Self(0x8b65);
    pub const GL_SAMPLER_CUBE_SHADOW: Self = Self(0x8dc5);
    pub const GL_SAMPLER_2D_ARRAY: Self = Self(0x8dc1);
    pub const GL_UNSIGNED_INT_SAMPLER_2D_ARRAY: Self = Self(0x8dd7);
    pub const GL_UNSIGNED_INT_SAMPLER_1D_ARRAY: Self = Self(0x8dd6);
    pub const GL_FLOAT_VEC3: Self = Self(0x8b51);
    pub const GL_FLOAT_MAT4x3: Self = Self(0x8b6a);
    pub const GL_SAMPLER_1D_ARRAY: Self = Self(0x8dc0);
    pub const GL_INT_SAMPLER_1D: Self = Self(0x8dc9);
    pub const GL_INT_SAMPLER_1D_ARRAY: Self = Self(0x8dce);
    pub const GL_INT_SAMPLER_2D_ARRAY: Self = Self(0x8dcf);
    pub const GL_DOUBLE_MAT2: Self = Self(0x8f46);
    pub const GL_BOOL_VEC3: Self = Self(0x8b58);
    pub const GL_BOOL_VEC4: Self = Self(0x8b59);
    pub const GL_DOUBLE_MAT4x2: Self = Self(0x8f4d);
    pub const GL_SAMPLER_CUBE_MAP_ARRAY: Self = Self(0x900c);
    pub const GL_UNSIGNED_INT_SAMPLER_2D_MULTISAMPLE: Self = Self(0x910a);
    pub const GL_SAMPLER_2D_MULTISAMPLE_ARRAY: Self = Self(0x910b);
    pub const GL_FLOAT_MAT3x4: Self = Self(0x8b68);
    pub const GL_INT_SAMPLER_2D_MULTISAMPLE: Self = Self(0x9109);
    pub const GL_FLOAT_MAT4x2: Self = Self(0x8b69);
    pub const GL_FLOAT_MAT3: Self = Self(0x8b5b);
    pub const GL_INT_SAMPLER_BUFFER: Self = Self(0x8dd0);
    pub const GL_INT: Self = Self(0x1404);
    pub const GL_DOUBLE_VEC3: Self = Self(0x8ffd);
    pub const GL_DOUBLE_MAT3: Self = Self(0x8f47);
    pub const GL_DOUBLE_MAT3x4: Self = Self(0x8f4c);
    pub const GL_SAMPLER_1D_ARRAY_SHADOW: Self = Self(0x8dc3);
    pub const GL_UNSIGNED_INT_VEC2: Self = Self(0x8dc6);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct HistogramTargetEXT(pub(crate) u64);
impl HistogramTargetEXT {
    pub const GL_PROXY_HISTOGRAM: Self = Self(0x8025);
    pub const GL_HISTOGRAM: Self = Self(0x8024);
    pub const GL_PROXY_HISTOGRAM_EXT: Self = Self(0x8025);
    pub const GL_HISTOGRAM_EXT: Self = Self(0x8024);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct IndexPointerType(pub(crate) u64);
impl IndexPointerType {
    pub const GL_FLOAT: Self = Self(0x1406);
    pub const GL_SHORT: Self = Self(0x1402);
    pub const GL_INT: Self = Self(0x1404);
    pub const GL_DOUBLE: Self = Self(0x140a);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TextureCompareMode(pub(crate) u64);
impl TextureCompareMode {
    pub const GL_NONE: Self = Self(0);
    pub const GL_COMPARE_R_TO_TEXTURE: Self = Self(0x884e);
    pub const GL_COMPARE_REF_TO_TEXTURE: Self = Self(0x884e);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TextureSwizzle(pub(crate) u64);
impl TextureSwizzle {
    pub const GL_GREEN: Self = Self(0x1904);
    pub const GL_BLUE: Self = Self(0x1905);
    pub const GL_RED: Self = Self(0x1903);
    pub const GL_ALPHA: Self = Self(0x1906);
    pub const GL_ONE: Self = Self(1);
    pub const GL_ZERO: Self = Self(0);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct InterleavedArrayFormat(pub(crate) u64);
impl InterleavedArrayFormat {
    pub const GL_T2F_C4UB_V3F: Self = Self(0x2a29);
    pub const GL_V2F: Self = Self(0x2a20);
    pub const GL_C4F_N3F_V3F: Self = Self(0x2a26);
    pub const GL_V3F: Self = Self(0x2a21);
    pub const GL_T2F_C3F_V3F: Self = Self(0x2a2a);
    pub const GL_T2F_V3F: Self = Self(0x2a27);
    pub const GL_C4UB_V3F: Self = Self(0x2a23);
    pub const GL_N3F_V3F: Self = Self(0x2a25);
    pub const GL_C4UB_V2F: Self = Self(0x2a22);
    pub const GL_T4F_V4F: Self = Self(0x2a28);
    pub const GL_C3F_V3F: Self = Self(0x2a24);
    pub const GL_T2F_N3F_V3F: Self = Self(0x2a2b);
    pub const GL_T2F_C4F_N3F_V3F: Self = Self(0x2a2c);
    pub const GL_T4F_C4F_N3F_V4F: Self = Self(0x2a2d);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PixelTexGenModeSGIX(pub(crate) u64);
impl PixelTexGenModeSGIX {
    pub const GL_PIXEL_TEX_GEN_Q_ROUND_SGIX: Self = Self(0x8185);
    pub const GL_PIXEL_TEX_GEN_Q_CEILING_SGIX: Self = Self(0x8184);
    pub const GL_PIXEL_TEX_GEN_ALPHA_LS_SGIX: Self = Self(0x8189);
    pub const GL_PIXEL_TEX_GEN_ALPHA_MS_SGIX: Self = Self(0x818a);
    pub const GL_PIXEL_TEX_GEN_Q_FLOOR_SGIX: Self = Self(0x8186);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct QueryTarget(pub(crate) u64);
impl QueryTarget {
    pub const GL_VERTICES_SUBMITTED: Self = Self(0x82ee);
    pub const GL_ANY_SAMPLES_PASSED: Self = Self(0x8c2f);
    pub const GL_PRIMITIVES_SUBMITTED: Self = Self(0x82ef);
    pub const GL_PRIMITIVES_GENERATED: Self = Self(0x8c87);
    pub const GL_TIME_ELAPSED: Self = Self(0x88bf);
    pub const GL_SAMPLES_PASSED: Self = Self(0x8914);
    pub const GL_TRANSFORM_FEEDBACK_PRIMITIVES_WRITTEN: Self = Self(0x8c88);
    pub const GL_ANY_SAMPLES_PASSED_CONSERVATIVE: Self = Self(0x8d6a);
    pub const GL_TRANSFORM_FEEDBACK_OVERFLOW: Self = Self(0x82ec);
    pub const GL_VERTEX_SHADER_INVOCATIONS: Self = Self(0x82f0);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SpecialNumbers(pub(crate) u64);
impl SpecialNumbers {
    pub const GL_TIMEOUT_IGNORED_APPLE: Self = Self(0xffff_ffff_ffff_ffff);
    pub const GL_VERSION_ES_CL_1_0: Self = Self(1);
    pub const GL_ONE: Self = Self(1);
    pub const GL_ZERO: Self = Self(0);
    pub const GL_TIMEOUT_IGNORED: Self = Self(0xffff_ffff_ffff_ffff);
    pub const GL_VERSION_ES_CM_1_1: Self = Self(1);
    pub const GL_ALL_PIXELS_AMD: Self = Self(0xffff_ffff);
    pub const GL_INVALID_INDEX: Self = Self(0xffff_ffff);
    pub const GL_VERSION_ES_CL_1_1: Self = Self(1);
    pub const GL_LUID_SIZE_EXT: Self = Self(8);
    pub const GL_UUID_SIZE_EXT: Self = Self(16);
    pub const GL_NO_ERROR: Self = Self(0);
    pub const GL_NONE: Self = Self(0);
    pub const GL_FALSE: Self = Self(0);
    pub const GL_TRUE: Self = Self(1);
    pub const GL_NONE_OES: Self = Self(0);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TextureEnvTarget(pub(crate) u64);
impl TextureEnvTarget {
    pub const GL_TEXTURE_ENV: Self = Self(0x2300);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ShaderType(pub(crate) u64);
impl ShaderType {
    pub const GL_FRAGMENT_SHADER: Self = Self(0x8b30);
    pub const GL_GEOMETRY_SHADER: Self = Self(0x8dd9);
    pub const GL_VERTEX_SHADER_ARB: Self = Self(0x8b31);
    pub const GL_TESS_CONTROL_SHADER: Self = Self(0x8e88);
    pub const GL_FRAGMENT_SHADER_ARB: Self = Self(0x8b30);
    pub const GL_COMPUTE_SHADER: Self = Self(0x91b9);
    pub const GL_VERTEX_SHADER: Self = Self(0x8b31);
    pub const GL_TESS_EVALUATION_SHADER: Self = Self(0x8e87);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PathCoverMode(pub(crate) u64);
impl PathCoverMode {
    pub const GL_BOUNDING_BOX_NV: Self = Self(0x908d);
    pub const GL_PATH_FILL_COVER_MODE_NV: Self = Self(0x9082);
    pub const GL_CONVEX_HULL_NV: Self = Self(0x908b);
    pub const GL_BOUNDING_BOX_OF_BOUNDING_BOXES_NV: Self = Self(0x909c);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ClampColorModeARB(pub(crate) u64);
impl ClampColorModeARB {
    pub const GL_FIXED_ONLY_ARB: Self = Self(0x891d);
    pub const GL_FIXED_ONLY: Self = Self(0x891d);
    pub const GL_FALSE: Self = Self(0);
    pub const GL_TRUE: Self = Self(1);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PathColorFormat(pub(crate) u64);
impl PathColorFormat {
    pub const GL_RGB: Self = Self(0x1907);
    pub const GL_LUMINANCE_ALPHA: Self = Self(0x190a);
    pub const GL_INTENSITY: Self = Self(0x8049);
    pub const GL_ALPHA: Self = Self(0x1906);
    pub const GL_RGBA: Self = Self(0x1908);
    pub const GL_LUMINANCE: Self = Self(0x1909);
    pub const GL_NONE: Self = Self(0);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VertexWeightPointerTypeEXT(pub(crate) u64);
impl VertexWeightPointerTypeEXT {
    pub const GL_FLOAT: Self = Self(0x1406);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SamplePatternEXT(pub(crate) u64);
impl SamplePatternEXT {
    pub const GL_4PASS_2_EXT: Self = Self(0x80a6);
    pub const GL_4PASS_3_EXT: Self = Self(0x80a7);
    pub const GL_4PASS_1_EXT: Self = Self(0x80a5);
    pub const GL_4PASS_0_EXT: Self = Self(0x80a4);
    pub const GL_2PASS_0_EXT: Self = Self(0x80a2);
    pub const GL_1PASS_EXT: Self = Self(0x80a1);
    pub const GL_2PASS_1_EXT: Self = Self(0x80a3);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TextureFilterSGIS(pub(crate) u64);
impl TextureFilterSGIS {
    pub const GL_FILTER4_SGIS: Self = Self(0x8146);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FramebufferStatus(pub(crate) u64);
impl FramebufferStatus {
    pub const GL_FRAMEBUFFER_INCOMPLETE_ATTACHMENT: Self = Self(0x8cd6);
    pub const GL_FRAMEBUFFER_INCOMPLETE_READ_BUFFER: Self = Self(0x8cdc);
    pub const GL_FRAMEBUFFER_COMPLETE: Self = Self(0x8cd5);
    pub const GL_FRAMEBUFFER_INCOMPLETE_DRAW_BUFFER: Self = Self(0x8cdb);
    pub const GL_FRAMEBUFFER_UNSUPPORTED: Self = Self(0x8cdd);
    pub const GL_FRAMEBUFFER_INCOMPLETE_MISSING_ATTACHMENT: Self = Self(0x8cd7);
    pub const GL_FRAMEBUFFER_INCOMPLETE_MULTISAMPLE: Self = Self(0x8d56);
    pub const GL_FRAMEBUFFER_UNDEFINED: Self = Self(0x8219);
    pub const GL_FRAMEBUFFER_INCOMPLETE_LAYER_TARGETS: Self = Self(0x8da8);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CombinerStageNV(pub(crate) u64);
impl CombinerStageNV {
    pub const GL_COMBINER0_NV: Self = Self(0x8550);
    pub const GL_COMBINER2_NV: Self = Self(0x8552);
    pub const GL_COMBINER7_NV: Self = Self(0x8557);
    pub const GL_COMBINER4_NV: Self = Self(0x8554);
    pub const GL_COMBINER6_NV: Self = Self(0x8556);
    pub const GL_COMBINER1_NV: Self = Self(0x8551);
    pub const GL_COMBINER3_NV: Self = Self(0x8553);
    pub const GL_COMBINER5_NV: Self = Self(0x8555);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FogPointerTypeEXT(pub(crate) u64);
impl FogPointerTypeEXT {
    pub const GL_FLOAT: Self = Self(0x1406);
    pub const GL_DOUBLE: Self = Self(0x140a);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GetFramebufferParameter(pub(crate) u64);
impl GetFramebufferParameter {
    pub const GL_SAMPLE_BUFFERS: Self = Self(0x80a8);
    pub const GL_FRAMEBUFFER_DEFAULT_FIXED_SAMPLE_LOCATIONS: Self = Self(0x9314);
    pub const GL_DOUBLEBUFFER: Self = Self(0x0c32);
    pub const GL_STEREO: Self = Self(0x0c33);
    pub const GL_FRAMEBUFFER_DEFAULT_WIDTH: Self = Self(0x9310);
    pub const GL_IMPLEMENTATION_COLOR_READ_TYPE: Self = Self(0x8b9a);
    pub const GL_FRAMEBUFFER_DEFAULT_HEIGHT: Self = Self(0x9311);
    pub const GL_FRAMEBUFFER_DEFAULT_LAYERS: Self = Self(0x9312);
    pub const GL_IMPLEMENTATION_COLOR_READ_FORMAT: Self = Self(0x8b9b);
    pub const GL_SAMPLES: Self = Self(0x80a9);
    pub const GL_FRAMEBUFFER_DEFAULT_SAMPLES: Self = Self(0x9313);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VertexShaderTextureUnitParameter(pub(crate) u64);
impl VertexShaderTextureUnitParameter {
    pub const GL_CURRENT_TEXTURE_COORDS: Self = Self(0x0b03);
    pub const GL_TEXTURE_MATRIX: Self = Self(0x0ba8);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TextureLayout(pub(crate) u64);
impl TextureLayout {
    pub const GL_LAYOUT_DEPTH_STENCIL_READ_ONLY_EXT: Self = Self(0x9590);
    pub const GL_LAYOUT_DEPTH_ATTACHMENT_STENCIL_READ_ONLY_EXT: Self = Self(0x9531);
    pub const GL_LAYOUT_DEPTH_READ_ONLY_STENCIL_ATTACHMENT_EXT: Self = Self(0x9530);
    pub const GL_LAYOUT_COLOR_ATTACHMENT_EXT: Self = Self(0x958e);
    pub const GL_LAYOUT_SHADER_READ_ONLY_EXT: Self = Self(0x9591);
    pub const GL_LAYOUT_DEPTH_STENCIL_ATTACHMENT_EXT: Self = Self(0x958f);
    pub const GL_LAYOUT_GENERAL_EXT: Self = Self(0x958d);
    pub const GL_LAYOUT_TRANSFER_SRC_EXT: Self = Self(0x9592);
    pub const GL_LAYOUT_TRANSFER_DST_EXT: Self = Self(0x9593);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ImageTransformPNameHP(pub(crate) u64);
impl ImageTransformPNameHP {
    pub const GL_IMAGE_TRANSLATE_X_HP: Self = Self(0x8157);
    pub const GL_IMAGE_ROTATE_ANGLE_HP: Self = Self(0x8159);
    pub const GL_IMAGE_SCALE_X_HP: Self = Self(0x8155);
    pub const GL_IMAGE_TRANSLATE_Y_HP: Self = Self(0x8158);
    pub const GL_IMAGE_CUBIC_WEIGHT_HP: Self = Self(0x815e);
    pub const GL_IMAGE_SCALE_Y_HP: Self = Self(0x8156);
    pub const GL_IMAGE_ROTATE_ORIGIN_Y_HP: Self = Self(0x815b);
    pub const GL_IMAGE_MIN_FILTER_HP: Self = Self(0x815d);
    pub const GL_IMAGE_MAG_FILTER_HP: Self = Self(0x815c);
    pub const GL_IMAGE_ROTATE_ORIGIN_X_HP: Self = Self(0x815a);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PixelDataRangeTargetNV(pub(crate) u64);
impl PixelDataRangeTargetNV {
    pub const GL_WRITE_PIXEL_DATA_RANGE_NV: Self = Self(0x8878);
    pub const GL_READ_PIXEL_DATA_RANGE_NV: Self = Self(0x8879);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GetTexBumpParameterATI(pub(crate) u64);
impl GetTexBumpParameterATI {
    pub const GL_BUMP_NUM_TEX_UNITS_ATI: Self = Self(0x8777);
    pub const GL_BUMP_ROT_MATRIX_SIZE_ATI: Self = Self(0x8776);
    pub const GL_BUMP_ROT_MATRIX_ATI: Self = Self(0x8775);
    pub const GL_BUMP_TEX_UNITS_ATI: Self = Self(0x8778);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PathFillMode(pub(crate) u64);
impl PathFillMode {
    pub const GL_COUNT_UP_NV: Self = Self(0x9088);
    pub const GL_PATH_FILL_MODE_NV: Self = Self(0x9080);
    pub const GL_COUNT_DOWN_NV: Self = Self(0x9089);
    pub const GL_INVERT: Self = Self(0x150a);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PixelType(pub(crate) u64);
impl PixelType {
    pub const GL_BYTE: Self = Self(0x1400);
    pub const GL_UNSIGNED_BYTE_3_3_2: Self = Self(0x8032);
    pub const GL_UNSIGNED_SHORT_5_5_5_1: Self = Self(0x8034);
    pub const GL_UNSIGNED_SHORT: Self = Self(0x1403);
    pub const GL_UNSIGNED_INT: Self = Self(0x1405);
    pub const GL_UNSIGNED_SHORT_4_4_4_4: Self = Self(0x8033);
    pub const GL_UNSIGNED_INT_10_10_10_2_EXT: Self = Self(0x8036);
    pub const GL_INT: Self = Self(0x1404);
    pub const GL_FLOAT: Self = Self(0x1406);
    pub const GL_UNSIGNED_INT_8_8_8_8: Self = Self(0x8035);
    pub const GL_SHORT: Self = Self(0x1402);
    pub const GL_UNSIGNED_INT_8_8_8_8_EXT: Self = Self(0x8035);
    pub const GL_UNSIGNED_INT_10_10_10_2: Self = Self(0x8036);
    pub const GL_UNSIGNED_SHORT_5_5_5_1_EXT: Self = Self(0x8034);
    pub const GL_UNSIGNED_SHORT_4_4_4_4_EXT: Self = Self(0x8033);
    pub const GL_UNSIGNED_BYTE_3_3_2_EXT: Self = Self(0x8032);
    pub const GL_BITMAP: Self = Self(0x1a00);
    pub const GL_UNSIGNED_BYTE: Self = Self(0x1401);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PatchParameterName(pub(crate) u64);
impl PatchParameterName {
    pub const GL_PATCH_DEFAULT_OUTER_LEVEL: Self = Self(0x8e74);
    pub const GL_PATCH_VERTICES: Self = Self(0x8e72);
    pub const GL_PATCH_DEFAULT_INNER_LEVEL: Self = Self(0x8e73);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PathTransformType(pub(crate) u64);
impl PathTransformType {
    pub const GL_TRANSPOSE_AFFINE_2D_NV: Self = Self(0x9096);
    pub const GL_AFFINE_3D_NV: Self = Self(0x9094);
    pub const GL_TRANSLATE_Y_NV: Self = Self(0x908f);
    pub const GL_TRANSPOSE_AFFINE_3D_NV: Self = Self(0x9098);
    pub const GL_TRANSLATE_X_NV: Self = Self(0x908e);
    pub const GL_TRANSLATE_2D_NV: Self = Self(0x9090);
    pub const GL_TRANSLATE_3D_NV: Self = Self(0x9091);
    pub const GL_AFFINE_2D_NV: Self = Self(0x9092);
    pub const GL_NONE: Self = Self(0);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VertexAttribLType(pub(crate) u64);
impl VertexAttribLType {
    pub const GL_DOUBLE: Self = Self(0x140a);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PixelStoreSubsampleRate(pub(crate) u64);
impl PixelStoreSubsampleRate {
    pub const GL_PIXEL_SUBSAMPLE_4444_SGIX: Self = Self(0x85a2);
    pub const GL_PIXEL_SUBSAMPLE_2424_SGIX: Self = Self(0x85a3);
    pub const GL_PIXEL_SUBSAMPLE_4242_SGIX: Self = Self(0x85a4);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MapTextureFormatINTEL(pub(crate) u64);
impl MapTextureFormatINTEL {
    pub const GL_LAYOUT_LINEAR_INTEL: Self = Self(1);
    pub const GL_LAYOUT_LINEAR_CPU_CACHED_INTEL: Self = Self(2);
    pub const GL_LAYOUT_DEFAULT_INTEL: Self = Self(0);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TextureFilterFuncSGIS(pub(crate) u64);
impl TextureFilterFuncSGIS {
    pub const GL_FILTER4_SGIS: Self = Self(0x8146);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FragmentOpATI(pub(crate) u64);
impl FragmentOpATI {
    pub const GL_LERP_ATI: Self = Self(0x8969);
    pub const GL_SUB_ATI: Self = Self(0x8965);
    pub const GL_DOT3_ATI: Self = Self(0x8966);
    pub const GL_MAD_ATI: Self = Self(0x8968);
    pub const GL_CND_ATI: Self = Self(0x896a);
    pub const GL_DOT2_ADD_ATI: Self = Self(0x896c);
    pub const GL_MUL_ATI: Self = Self(0x8964);
    pub const GL_DOT4_ATI: Self = Self(0x8967);
    pub const GL_ADD_ATI: Self = Self(0x8963);
    pub const GL_MOV_ATI: Self = Self(0x8961);
    pub const GL_CND0_ATI: Self = Self(0x896b);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FenceConditionNV(pub(crate) u64);
impl FenceConditionNV {
    pub const GL_ALL_COMPLETED_NV: Self = Self(0x84f2);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CombinerMappingNV(pub(crate) u64);
impl CombinerMappingNV {
    pub const GL_UNSIGNED_INVERT_NV: Self = Self(0x8537);
    pub const GL_HALF_BIAS_NEGATE_NV: Self = Self(0x853b);
    pub const GL_EXPAND_NEGATE_NV: Self = Self(0x8539);
    pub const GL_SIGNED_IDENTITY_NV: Self = Self(0x853c);
    pub const GL_SIGNED_NEGATE_NV: Self = Self(0x853d);
    pub const GL_UNSIGNED_IDENTITY_NV: Self = Self(0x8536);
    pub const GL_EXPAND_NORMAL_NV: Self = Self(0x8538);
    pub const GL_HALF_BIAS_NORMAL_NV: Self = Self(0x853a);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TriangleListSUN(pub(crate) u64);
impl TriangleListSUN {
    pub const GL_RESTART_SUN: Self = Self(0x0001);
    pub const GL_REPLACE_MIDDLE_SUN: Self = Self(0x0002);
    pub const GL_REPLACE_OLDEST_SUN: Self = Self(0x0003);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FragmentLightParameterSGIX(pub(crate) u64);
impl FragmentLightParameterSGIX {
    pub const GL_SPOT_CUTOFF: Self = Self(0x1206);
    pub const GL_AMBIENT: Self = Self(0x1200);
    pub const GL_DIFFUSE: Self = Self(0x1201);
    pub const GL_LINEAR_ATTENUATION: Self = Self(0x1208);
    pub const GL_SPOT_DIRECTION: Self = Self(0x1204);
    pub const GL_POSITION: Self = Self(0x1203);
    pub const GL_SPOT_EXPONENT: Self = Self(0x1205);
    pub const GL_CONSTANT_ATTENUATION: Self = Self(0x1207);
    pub const GL_SPECULAR: Self = Self(0x1202);
    pub const GL_QUADRATIC_ATTENUATION: Self = Self(0x1209);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TexBumpParameterATI(pub(crate) u64);
impl TexBumpParameterATI {
    pub const GL_BUMP_ROT_MATRIX_ATI: Self = Self(0x8775);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BlendingFactor(pub(crate) u64);
impl BlendingFactor {
    pub const GL_CONSTANT_COLOR: Self = Self(0x8001);
    pub const GL_CONSTANT_ALPHA: Self = Self(0x8003);
    pub const GL_DST_ALPHA: Self = Self(0x0304);
    pub const GL_DST_COLOR: Self = Self(0x0306);
    pub const GL_ZERO: Self = Self(0);
    pub const GL_SRC_COLOR: Self = Self(0x0300);
    pub const GL_SRC_ALPHA_SATURATE: Self = Self(0x0308);
    pub const GL_ONE_MINUS_SRC_ALPHA: Self = Self(0x0303);
    pub const GL_ONE_MINUS_CONSTANT_COLOR: Self = Self(0x8002);
    pub const GL_ONE_MINUS_CONSTANT_ALPHA: Self = Self(0x8004);
    pub const GL_SRC1_COLOR: Self = Self(0x88f9);
    pub const GL_ONE_MINUS_SRC_COLOR: Self = Self(0x0301);
    pub const GL_SRC_ALPHA: Self = Self(0x0302);
    pub const GL_ONE_MINUS_SRC1_COLOR: Self = Self(0x88fa);
    pub const GL_ONE: Self = Self(1);
    pub const GL_ONE_MINUS_DST_ALPHA: Self = Self(0x0305);
    pub const GL_SRC1_ALPHA: Self = Self(0x8589);
    pub const GL_ONE_MINUS_SRC1_ALPHA: Self = Self(0x88fb);
    pub const GL_ONE_MINUS_DST_COLOR: Self = Self(0x0307);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TextureMagFilter(pub(crate) u64);
impl TextureMagFilter {
    pub const GL_LINEAR: Self = Self(0x2601);
    pub const GL_LINEAR_DETAIL_ALPHA_SGIS: Self = Self(0x8098);
    pub const GL_PIXEL_TEX_GEN_Q_ROUND_SGIX: Self = Self(0x8185);
    pub const GL_NEAREST: Self = Self(0x2600);
    pub const GL_LINEAR_SHARPEN_ALPHA_SGIS: Self = Self(0x80ae);
    pub const GL_PIXEL_TEX_GEN_Q_FLOOR_SGIX: Self = Self(0x8186);
    pub const GL_LINEAR_DETAIL_SGIS: Self = Self(0x8097);
    pub const GL_LINEAR_SHARPEN_COLOR_SGIS: Self = Self(0x80af);
    pub const GL_PIXEL_TEX_GEN_Q_CEILING_SGIX: Self = Self(0x8184);
    pub const GL_LINEAR_DETAIL_COLOR_SGIS: Self = Self(0x8099);
    pub const GL_FILTER4_SGIS: Self = Self(0x8146);
    pub const GL_LINEAR_SHARPEN_SGIS: Self = Self(0x80ad);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ColorTableParameterPNameSGI(pub(crate) u64);
impl ColorTableParameterPNameSGI {
    pub const GL_COLOR_TABLE_WIDTH: Self = Self(0x80d9);
    pub const GL_COLOR_TABLE_ALPHA_SIZE: Self = Self(0x80dd);
    pub const GL_COLOR_TABLE_GREEN_SIZE: Self = Self(0x80db);
    pub const GL_COLOR_TABLE_RED_SIZE: Self = Self(0x80da);
    pub const GL_COLOR_TABLE_BLUE_SIZE_SGI: Self = Self(0x80dc);
    pub const GL_COLOR_TABLE_BLUE_SIZE: Self = Self(0x80dc);
    pub const GL_COLOR_TABLE_BIAS_SGI: Self = Self(0x80d7);
    pub const GL_COLOR_TABLE_GREEN_SIZE_SGI: Self = Self(0x80db);
    pub const GL_COLOR_TABLE_FORMAT: Self = Self(0x80d8);
    pub const GL_COLOR_TABLE_LUMINANCE_SIZE: Self = Self(0x80de);
    pub const GL_COLOR_TABLE_INTENSITY_SIZE_SGI: Self = Self(0x80df);
    pub const GL_COLOR_TABLE_BIAS: Self = Self(0x80d7);
    pub const GL_COLOR_TABLE_SCALE_SGI: Self = Self(0x80d6);
    pub const GL_COLOR_TABLE_RED_SIZE_SGI: Self = Self(0x80da);
    pub const GL_COLOR_TABLE_WIDTH_SGI: Self = Self(0x80d9);
    pub const GL_COLOR_TABLE_ALPHA_SIZE_SGI: Self = Self(0x80dd);
    pub const GL_COLOR_TABLE_INTENSITY_SIZE: Self = Self(0x80df);
    pub const GL_COLOR_TABLE_SCALE: Self = Self(0x80d6);
    pub const GL_COLOR_TABLE_LUMINANCE_SIZE_SGI: Self = Self(0x80de);
    pub const GL_COLOR_TABLE_FORMAT_SGI: Self = Self(0x80d8);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VertexAttribPropertyARB(pub(crate) u64);
impl VertexAttribPropertyARB {
    pub const GL_VERTEX_ATTRIB_ARRAY_NORMALIZED: Self = Self(0x886a);
    pub const GL_VERTEX_ATTRIB_ARRAY_BUFFER_BINDING: Self = Self(0x889f);
    pub const GL_VERTEX_ATTRIB_ARRAY_INTEGER_EXT: Self = Self(0x88fd);
    pub const GL_VERTEX_ATTRIB_ARRAY_DIVISOR: Self = Self(0x88fe);
    pub const GL_VERTEX_ATTRIB_ARRAY_INTEGER: Self = Self(0x88fd);
    pub const GL_VERTEX_ATTRIB_RELATIVE_OFFSET: Self = Self(0x82d5);
    pub const GL_VERTEX_ATTRIB_ARRAY_STRIDE: Self = Self(0x8624);
    pub const GL_VERTEX_ATTRIB_ARRAY_TYPE: Self = Self(0x8625);
    pub const GL_VERTEX_ATTRIB_ARRAY_ENABLED: Self = Self(0x8622);
    pub const GL_CURRENT_VERTEX_ATTRIB: Self = Self(0x8626);
    pub const GL_VERTEX_ATTRIB_ARRAY_SIZE: Self = Self(0x8623);
    pub const GL_VERTEX_ATTRIB_BINDING: Self = Self(0x82d4);
    pub const GL_VERTEX_ATTRIB_ARRAY_LONG: Self = Self(0x874e);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MapParameterNV(pub(crate) u64);
impl MapParameterNV {
    pub const GL_MAP_TESSELLATION_NV: Self = Self(0x86c2);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VertexAttribPointerType(pub(crate) u64);
impl VertexAttribPointerType {
    pub const GL_UNSIGNED_INT64_NV: Self = Self(0x140f);
    pub const GL_DOUBLE: Self = Self(0x140a);
    pub const GL_UNSIGNED_INT_10F_11F_11F_REV: Self = Self(0x8c3b);
    pub const GL_INT_2_10_10_10_REV: Self = Self(0x8d9f);
    pub const GL_UNSIGNED_INT_2_10_10_10_REV: Self = Self(0x8368);
    pub const GL_BYTE: Self = Self(0x1400);
    pub const GL_UNSIGNED_INT64_ARB: Self = Self(0x140f);
    pub const GL_UNSIGNED_BYTE: Self = Self(0x1401);
    pub const GL_FIXED: Self = Self(0x140c);
    pub const GL_UNSIGNED_INT: Self = Self(0x1405);
    pub const GL_FLOAT: Self = Self(0x1406);
    pub const GL_UNSIGNED_SHORT: Self = Self(0x1403);
    pub const GL_INT: Self = Self(0x1404);
    pub const GL_SHORT: Self = Self(0x1402);
    pub const GL_INT64_ARB: Self = Self(0x140e);
    pub const GL_HALF_FLOAT: Self = Self(0x140b);
    pub const GL_INT64_NV: Self = Self(0x140e);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ShaderParameterName(pub(crate) u64);
impl ShaderParameterName {
    pub const GL_INFO_LOG_LENGTH: Self = Self(0x8b84);
    pub const GL_COMPILE_STATUS: Self = Self(0x8b81);
    pub const GL_DELETE_STATUS: Self = Self(0x8b80);
    pub const GL_SHADER_SOURCE_LENGTH: Self = Self(0x8b88);
    pub const GL_SHADER_TYPE: Self = Self(0x8b4f);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TextureWrapMode(pub(crate) u64);
impl TextureWrapMode {
    pub const GL_CLAMP_TO_BORDER_ARB: Self = Self(0x812d);
    pub const GL_CLAMP: Self = Self(0x2900);
    pub const GL_CLAMP_TO_EDGE: Self = Self(0x812f);
    pub const GL_CLAMP_TO_BORDER_NV: Self = Self(0x812d);
    pub const GL_REPEAT: Self = Self(0x2901);
    pub const GL_CLAMP_TO_BORDER_SGIS: Self = Self(0x812d);
    pub const GL_LINEAR_MIPMAP_LINEAR: Self = Self(0x2703);
    pub const GL_MIRRORED_REPEAT: Self = Self(0x8370);
    pub const GL_CLAMP_TO_EDGE_SGIS: Self = Self(0x812f);
    pub const GL_CLAMP_TO_BORDER: Self = Self(0x812d);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CheckFramebufferStatusTarget(pub(crate) u64);
impl CheckFramebufferStatusTarget {
    pub const GL_READ_FRAMEBUFFER: Self = Self(0x8ca8);
    pub const GL_DRAW_FRAMEBUFFER: Self = Self(0x8ca9);
    pub const GL_FRAMEBUFFER: Self = Self(0x8d40);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SamplerParameterI(pub(crate) u64);
impl SamplerParameterI {
    pub const GL_TEXTURE_UNNORMALIZED_COORDINATES_ARM: Self = Self(0x8f6a);
    pub const GL_TEXTURE_WRAP_T: Self = Self(0x2803);
    pub const GL_TEXTURE_COMPARE_FUNC: Self = Self(0x884d);
    pub const GL_TEXTURE_MIN_FILTER: Self = Self(0x2801);
    pub const GL_TEXTURE_COMPARE_MODE: Self = Self(0x884c);
    pub const GL_TEXTURE_MAG_FILTER: Self = Self(0x2800);
    pub const GL_TEXTURE_WRAP_S: Self = Self(0x2802);
    pub const GL_TEXTURE_WRAP_R: Self = Self(0x8072);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PointParameterNameSGIS(pub(crate) u64);
impl PointParameterNameSGIS {
    pub const GL_POINT_SIZE_MAX_ARB: Self = Self(0x8127);
    pub const GL_POINT_SIZE_MIN_ARB: Self = Self(0x8126);
    pub const GL_POINT_FADE_THRESHOLD_SIZE_SGIS: Self = Self(0x8128);
    pub const GL_POINT_SIZE_MAX_SGIS: Self = Self(0x8127);
    pub const GL_POINT_SIZE_MIN: Self = Self(0x8126);
    pub const GL_POINT_DISTANCE_ATTENUATION_ARB: Self = Self(0x8129);
    pub const GL_POINT_FADE_THRESHOLD_SIZE_ARB: Self = Self(0x8128);
    pub const GL_DISTANCE_ATTENUATION_SGIS: Self = Self(0x8129);
    pub const GL_POINT_SIZE_MIN_SGIS: Self = Self(0x8126);
    pub const GL_POINT_SIZE_MIN_EXT: Self = Self(0x8126);
    pub const GL_POINT_FADE_THRESHOLD_SIZE: Self = Self(0x8128);
    pub const GL_DISTANCE_ATTENUATION_EXT: Self = Self(0x8129);
    pub const GL_POINT_DISTANCE_ATTENUATION: Self = Self(0x8129);
    pub const GL_POINT_SIZE_MAX_EXT: Self = Self(0x8127);
    pub const GL_POINT_SIZE_MAX: Self = Self(0x8127);
    pub const GL_POINT_FADE_THRESHOLD_SIZE_EXT: Self = Self(0x8128);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct HintTargetPGI(pub(crate) u64);
impl HintTargetPGI {
    pub const GL_MATERIAL_SIDE_HINT_PGI: Self = Self(0x1_a22c);
    pub const GL_VERTEX_DATA_HINT_PGI: Self = Self(0x1_a22a);
    pub const GL_MAX_VERTEX_HINT_PGI: Self = Self(0x1_a22d);
    pub const GL_VERTEX_CONSISTENT_HINT_PGI: Self = Self(0x1_a22b);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct QueryObjectParameterName(pub(crate) u64);
impl QueryObjectParameterName {
    pub const GL_QUERY_RESULT_NO_WAIT: Self = Self(0x9194);
    pub const GL_QUERY_TARGET: Self = Self(0x82ea);
    pub const GL_QUERY_RESULT_AVAILABLE: Self = Self(0x8867);
    pub const GL_QUERY_RESULT: Self = Self(0x8866);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FragmentLightModelParameterSGIX(pub(crate) u64);
impl FragmentLightModelParameterSGIX {
    pub const GL_FRAGMENT_LIGHT_MODEL_LOCAL_VIEWER_SGIX: Self = Self(0x8408);
    pub const GL_FRAGMENT_LIGHT_MODEL_NORMAL_INTERPOLATION_SGIX: Self = Self(0x840b);
    pub const GL_FRAGMENT_LIGHT_MODEL_TWO_SIDE_SGIX: Self = Self(0x8409);
    pub const GL_FRAGMENT_LIGHT_MODEL_AMBIENT_SGIX: Self = Self(0x840a);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PathGenMode(pub(crate) u64);
impl PathGenMode {
    pub const GL_NONE: Self = Self(0);
    pub const GL_EYE_LINEAR: Self = Self(0x2400);
    pub const GL_OBJECT_LINEAR: Self = Self(0x2401);
    pub const GL_PATH_OBJECT_BOUNDING_BOX_NV: Self = Self(0x908a);
    pub const GL_CONSTANT: Self = Self(0x8576);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ProgramInterfacePName(pub(crate) u64);
impl ProgramInterfacePName {
    pub const GL_MAX_NAME_LENGTH: Self = Self(0x92f6);
    pub const GL_ACTIVE_RESOURCES: Self = Self(0x92f5);
    pub const GL_MAX_NUM_ACTIVE_VARIABLES: Self = Self(0x92f7);
    pub const GL_MAX_NUM_COMPATIBLE_SUBROUTINES: Self = Self(0x92f8);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ExternalHandleType(pub(crate) u64);
impl ExternalHandleType {
    pub const GL_HANDLE_TYPE_OPAQUE_WIN32_EXT: Self = Self(0x9587);
    pub const GL_HANDLE_TYPE_OPAQUE_FD_EXT: Self = Self(0x9586);
    pub const GL_HANDLE_TYPE_D3D12_TILEPOOL_EXT: Self = Self(0x9589);
    pub const GL_HANDLE_TYPE_D3D11_IMAGE_EXT: Self = Self(0x958b);
    pub const GL_HANDLE_TYPE_D3D11_IMAGE_KMT_EXT: Self = Self(0x958c);
    pub const GL_HANDLE_TYPE_D3D12_RESOURCE_EXT: Self = Self(0x958a);
    pub const GL_HANDLE_TYPE_D3D12_FENCE_EXT: Self = Self(0x9594);
    pub const GL_HANDLE_TYPE_OPAQUE_WIN32_KMT_EXT: Self = Self(0x9588);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CombinerPortionNV(pub(crate) u64);
impl CombinerPortionNV {
    pub const GL_RGB: Self = Self(0x1907);
    pub const GL_ALPHA: Self = Self(0x1906);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FogMode(pub(crate) u64);
impl FogMode {
    pub const GL_FOG_FUNC_SGIS: Self = Self(0x812a);
    pub const GL_LINEAR: Self = Self(0x2601);
    pub const GL_EXP2: Self = Self(0x0801);
    pub const GL_EXP: Self = Self(0x0800);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FeedBackToken(pub(crate) u64);
impl FeedBackToken {
    pub const GL_COPY_PIXEL_TOKEN: Self = Self(0x0706);
    pub const GL_LINE_RESET_TOKEN: Self = Self(0x0707);
    pub const GL_PASS_THROUGH_TOKEN: Self = Self(0x0700);
    pub const GL_DRAW_PIXEL_TOKEN: Self = Self(0x0705);
    pub const GL_BITMAP_TOKEN: Self = Self(0x0704);
    pub const GL_POLYGON_TOKEN: Self = Self(0x0703);
    pub const GL_LINE_TOKEN: Self = Self(0x0702);
    pub const GL_POINT_TOKEN: Self = Self(0x0701);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct InternalFormat(pub(crate) u64);
impl InternalFormat {
    pub const GL_COMPRESSED_SRGB8_PUNCHTHROUGH_ALPHA1_ETC2_OES: Self = Self(0x9277);
    pub const GL_QUAD_ALPHA4_SGIS: Self = Self(0x811e);
    pub const GL_RG32F_EXT: Self = Self(0x8230);
    pub const GL_LUMINANCE4: Self = Self(0x803f);
    pub const GL_LUMINANCE12: Self = Self(0x8041);
    pub const GL_RG16UI: Self = Self(0x823a);
    pub const GL_STENCIL_INDEX_OES: Self = Self(0x1901);
    pub const GL_RGB16F_ARB: Self = Self(0x881b);
    pub const GL_RGBA8UI_EXT: Self = Self(0x8d7c);
    pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_5x5x4_OES: Self = Self(0x93e5);
    pub const GL_RGBA32F_EXT: Self = Self(0x8814);
    pub const GL_RGBA16F_EXT: Self = Self(0x881a);
    pub const GL_RGBA32F_ARB: Self = Self(0x8814);
    pub const GL_RGBA16UI_EXT: Self = Self(0x8d76);
    pub const GL_RG16_SNORM: Self = Self(0x8f99);
    pub const GL_ALPHA8UI_EXT: Self = Self(0x8d7e);
    pub const GL_RGB8_EXT: Self = Self(0x8051);
    pub const GL_RGB32F: Self = Self(0x8815);
    pub const GL_DEPTH24_STENCIL8_OES: Self = Self(0x88f0);
    pub const GL_COMPRESSED_RGBA_ASTC_6x6: Self = Self(0x93b4);
    pub const GL_R8_EXT: Self = Self(0x8229);
    pub const GL_COMPRESSED_RGB8_PUNCHTHROUGH_ALPHA1_ETC2: Self = Self(0x9276);
    pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_5x4_KHR: Self = Self(0x93d1);
    pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_6x6: Self = Self(0x93d4);
    pub const GL_COMPRESSED_RGBA_ASTC_10x8: Self = Self(0x93ba);
    pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_6x5: Self = Self(0x93d3);
    pub const GL_RGBA16I: Self = Self(0x8d88);
    pub const GL_RED_EXT: Self = Self(0x1903);
    pub const GL_COMPRESSED_SRGB_ALPHA_BPTC_UNORM: Self = Self(0x8e8d);
    pub const GL_STENCIL_INDEX1_OES: Self = Self(0x8d46);
    pub const GL_LUMINANCE16_ALPHA16: Self = Self(0x8048);
    pub const GL_RGB: Self = Self(0x1907);
    pub const GL_COMPRESSED_RGBA_S3TC_DXT1_EXT: Self = Self(0x83f1);
    pub const GL_COMPRESSED_RGBA_ASTC_12x12: Self = Self(0x93bd);
    pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_10x5: Self = Self(0x93d8);
    pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_6x6x6_OES: Self = Self(0x93e9);
    pub const GL_COMPRESSED_RGBA_ASTC_6x6x5_OES: Self = Self(0x93c8);
    pub const GL_RGBA16_SNORM: Self = Self(0x8f9b);
    pub const GL_SRGB8_ALPHA8_EXT: Self = Self(0x8c43);
    pub const GL_SR8_EXT: Self = Self(0x8fbd);
    pub const GL_COMPRESSED_RGBA_ASTC_10x5_KHR: Self = Self(0x93b8);
    pub const GL_DEPTH_COMPONENT24_ARB: Self = Self(0x81a6);
    pub const GL_COMPRESSED_SRGB8_ALPHA8_ETC2_EAC_OES: Self = Self(0x9279);
    pub const GL_COMPRESSED_SRGB: Self = Self(0x8c48);
    pub const GL_RGB32I: Self = Self(0x8d83);
    pub const GL_RGB8_OES: Self = Self(0x8051);
    pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_8x5: Self = Self(0x93d5);
    pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_8x6_KHR: Self = Self(0x93d6);
    pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_4x4: Self = Self(0x93d0);
    pub const GL_COMPRESSED_SRGB_S3TC_DXT1_NV: Self = Self(0x8c4c);
    pub const GL_R8I: Self = Self(0x8231);
    pub const GL_COMPRESSED_RGBA_ASTC_3x3x3_OES: Self = Self(0x93c0);
    pub const GL_RG16F: Self = Self(0x822f);
    pub const GL_R16: Self = Self(0x822a);
    pub const GL_INTENSITY8I_EXT: Self = Self(0x8d91);
    pub const GL_DUAL_ALPHA12_SGIS: Self = Self(0x8112);
    pub const GL_LUMINANCE4_ALPHA4_OES: Self = Self(0x8043);
    pub const GL_RGB32F_EXT: Self = Self(0x8815);
    pub const GL_RG8: Self = Self(0x822b);
    pub const GL_COMPRESSED_RGBA_ASTC_6x6_KHR: Self = Self(0x93b4);
    pub const GL_DEPTH_COMPONENT32F_NV: Self = Self(0x8dab);
    pub const GL_ALPHA4: Self = Self(0x803b);
    pub const GL_RG32I: Self = Self(0x823b);
    pub const GL_DEPTH24_STENCIL8: Self = Self(0x88f0);
    pub const GL_LUMINANCE12_ALPHA12_EXT: Self = Self(0x8047);
    pub const GL_COMPRESSED_RGB8_ETC2: Self = Self(0x9274);
    pub const GL_DUAL_ALPHA16_SGIS: Self = Self(0x8113);
    pub const GL_QUAD_INTENSITY8_SGIS: Self = Self(0x8123);
    pub const GL_LUMINANCE16I_EXT: Self = Self(0x8d8c);
    pub const GL_COMPRESSED_RG_RGTC2: Self = Self(0x8dbd);
    pub const GL_COMPRESSED_SRGB8_ALPHA8_ETC2_EAC: Self = Self(0x9279);
    pub const GL_LUMINANCE12_ALPHA4_EXT: Self = Self(0x8046);
    pub const GL_COMPRESSED_RGBA_ASTC_8x5: Self = Self(0x93b5);
    pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_10x8: Self = Self(0x93da);
    pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_6x6_KHR: Self = Self(0x93d4);
    pub const GL_DUAL_LUMINANCE_ALPHA4_SGIS: Self = Self(0x811c);
    pub const GL_ALPHA8I_EXT: Self = Self(0x8d90);
    pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_8x8: Self = Self(0x93d7);
    pub const GL_RG16_EXT: Self = Self(0x822c);
    pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_8x5_KHR: Self = Self(0x93d5);
    pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_12x10_KHR: Self = Self(0x93dc);
    pub const GL_SRGB_ALPHA: Self = Self(0x8c42);
    pub const GL_INTENSITY16UI_EXT: Self = Self(0x8d79);
    pub const GL_DUAL_INTENSITY4_SGIS: Self = Self(0x8118);
    pub const GL_LUMINANCE32I_EXT: Self = Self(0x8d86);
    pub const GL_COMPRESSED_RGBA_ASTC_4x4_KHR: Self = Self(0x93b0);
    pub const GL_COMPRESSED_RGBA_ASTC_5x5x4_OES: Self = Self(0x93c5);
    pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_6x5x5_OES: Self = Self(0x93e7);
    pub const GL_INTENSITY4_EXT: Self = Self(0x804a);
    pub const GL_INTENSITY8: Self = Self(0x804b);
    pub const GL_COMPRESSED_RGBA_S3TC_DXT3_EXT: Self = Self(0x83f2);
    pub const GL_RG32F: Self = Self(0x8230);
    pub const GL_DEPTH32F_STENCIL8_NV: Self = Self(0x8dac);
    pub const GL_DEPTH_COMPONENT16_SGIX: Self = Self(0x81a5);
    pub const GL_R16_EXT: Self = Self(0x822a);
    pub const GL_R8UI: Self = Self(0x8232);
    pub const GL_COMPRESSED_SIGNED_RED_GREEN_RGTC2_EXT: Self = Self(0x8dbe);
    pub const GL_COMPRESSED_RED_GREEN_RGTC2_EXT: Self = Self(0x8dbd);
    pub const GL_LUMINANCE4_EXT: Self = Self(0x803f);
    pub const GL_RG8_SNORM: Self = Self(0x8f95);
    pub const GL_COMPRESSED_RG11_EAC: Self = Self(0x9272);
    pub const GL_COMPRESSED_SIGNED_RG11_EAC: Self = Self(0x9273);
    pub const GL_INTENSITY12: Self = Self(0x804c);
    pub const GL_COMPRESSED_SRGB8_PUNCHTHROUGH_ALPHA1_ETC2: Self = Self(0x9277);
    pub const GL_LUMINANCE12_EXT: Self = Self(0x8041);
    pub const GL_COMPRESSED_RGBA_ASTC_6x6x6_OES: Self = Self(0x93c9);
    pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_8x8_KHR: Self = Self(0x93d7);
    pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_12x12: Self = Self(0x93dd);
    pub const GL_COMPRESSED_RGBA_ASTC_12x12_KHR: Self = Self(0x93bd);
    pub const GL_RGB9_E5_APPLE: Self = Self(0x8c3d);
    pub const GL_R16_SNORM_EXT: Self = Self(0x8f98);
    pub const GL_DEPTH_COMPONENT24_SGIX: Self = Self(0x81a6);
    pub const GL_SRG8_EXT: Self = Self(0x8fbe);
    pub const GL_DEPTH_COMPONENT: Self = Self(0x1902);
    pub const GL_COMPRESSED_RGBA_S3TC_DXT3_ANGLE: Self = Self(0x83f2);
    pub const GL_RGBA32F: Self = Self(0x8814);
    pub const GL_RGBA2: Self = Self(0x8055);
    pub const GL_RGB16: Self = Self(0x8054);
    pub const GL_ALPHA16UI_EXT: Self = Self(0x8d78);
    pub const GL_COMPRESSED_RGBA_ASTC_10x6_KHR: Self = Self(0x93b9);
    pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_10x6_KHR: Self = Self(0x93d9);
    pub const GL_COMPRESSED_SRGB_ALPHA_S3TC_DXT3_NV: Self = Self(0x8c4e);
    pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_10x10: Self = Self(0x93db);
    pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_10x10_KHR: Self = Self(0x93db);
    pub const GL_COMPRESSED_RGBA: Self = Self(0x84ee);
    pub const GL_COMPRESSED_SRGB_ALPHA_S3TC_DXT1_NV: Self = Self(0x8c4d);
    pub const GL_LUMINANCE8_OES: Self = Self(0x8040);
    pub const GL_COMPRESSED_RGBA_ASTC_12x10: Self = Self(0x93bc);
    pub const GL_INTENSITY12_EXT: Self = Self(0x804c);
    pub const GL_COMPRESSED_RGB_BPTC_SIGNED_FLOAT_EXT: Self = Self(0x8e8e);
    pub const GL_RGBA8I_EXT: Self = Self(0x8d8e);
    pub const GL_COMPRESSED_RGBA_ASTC_4x4x4_OES: Self = Self(0x93c3);
    pub const GL_LUMINANCE8_ALPHA8_OES: Self = Self(0x8045);
    pub const GL_STENCIL_INDEX4_OES: Self = Self(0x8d47);
    pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_4x4x3_OES: Self = Self(0x93e2);
    pub const GL_ETC1_RGB8_OES: Self = Self(0x8d64);
    pub const GL_SRGB_ALPHA_EXT: Self = Self(0x8c42);
    pub const GL_COMPRESSED_RGBA_ASTC_10x5: Self = Self(0x93b8);
    pub const GL_R32UI: Self = Self(0x8236);
    pub const GL_RGB12_EXT: Self = Self(0x8053);
    pub const GL_ALPHA12_EXT: Self = Self(0x803d);
    pub const GL_RGBA16_EXT: Self = Self(0x805b);
    pub const GL_RGB5_EXT: Self = Self(0x8050);
    pub const GL_RGB5_A1: Self = Self(0x8057);
    pub const GL_DEPTH_COMPONENT16_ARB: Self = Self(0x81a5);
    pub const GL_DEPTH32F_STENCIL8: Self = Self(0x8cad);
    pub const GL_RGBA32UI: Self = Self(0x8d70);
    pub const GL_R16F: Self = Self(0x822d);
    pub const GL_DEPTH_COMPONENT32_OES: Self = Self(0x81a7);
    pub const GL_RGB16UI: Self = Self(0x8d77);
    pub const GL_COMPRESSED_RGBA_ASTC_8x6_KHR: Self = Self(0x93b6);
    pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_10x8_KHR: Self = Self(0x93da);
    pub const GL_STENCIL_INDEX4_EXT: Self = Self(0x8d47);
    pub const GL_LUMINANCE8_ALPHA8: Self = Self(0x8045);
    pub const GL_RGBA32I_EXT: Self = Self(0x8d82);
    pub const GL_STENCIL_INDEX1: Self = Self(0x8d46);
    pub const GL_LUMINANCE8_ALPHA8_EXT: Self = Self(0x8045);
    pub const GL_COMPRESSED_SIGNED_RED_RGTC1: Self = Self(0x8dbc);
    pub const GL_RGB5_A1_OES: Self = Self(0x8057);
    pub const GL_DEPTH_COMPONENT16: Self = Self(0x81a5);
    pub const GL_RGB565: Self = Self(0x8d62);
    pub const GL_RGBA8I: Self = Self(0x8d8e);
    pub const GL_INTENSITY16I_EXT: Self = Self(0x8d8b);
    pub const GL_STENCIL_INDEX8_EXT: Self = Self(0x8d48);
    pub const GL_LUMINANCE_ALPHA8I_EXT: Self = Self(0x8d93);
    pub const GL_COMPRESSED_RGBA_BPTC_UNORM: Self = Self(0x8e8c);
    pub const GL_R16_SNORM: Self = Self(0x8f98);
    pub const GL_COMPRESSED_RGBA_ASTC_5x5_KHR: Self = Self(0x93b2);
    pub const GL_COMPRESSED_RGBA_ASTC_8x5_KHR: Self = Self(0x93b5);
    pub const GL_RGB8UI: Self = Self(0x8d7d);
    pub const GL_RGBA32I: Self = Self(0x8d82);
    pub const GL_COMPRESSED_RGBA_ASTC_5x5x5_OES: Self = Self(0x93c6);
    pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_3x3x3_OES: Self = Self(0x93e0);
    pub const GL_LUMINANCE16_ALPHA16_EXT: Self = Self(0x8048);
    pub const GL_COMPRESSED_SRGB_ALPHA_BPTC_UNORM_EXT: Self = Self(0x8e8d);
    pub const GL_RGB16_SNORM_EXT: Self = Self(0x8f9a);
    pub const GL_COMPRESSED_RGBA8_ETC2_EAC_OES: Self = Self(0x9278);
    pub const GL_RGB4: Self = Self(0x804f);
    pub const GL_DEPTH_COMPONENT32F: Self = Self(0x8cac);
    pub const GL_RG: Self = Self(0x8227);
    pub const GL_RGB16UI_EXT: Self = Self(0x8d77);
    pub const GL_R32F: Self = Self(0x822e);
    pub const GL_COMPRESSED_SRGB_S3TC_DXT1_EXT: Self = Self(0x8c4c);
    pub const GL_COMPRESSED_RED: Self = Self(0x8225);
    pub const GL_RGB16F_EXT: Self = Self(0x881b);
    pub const GL_RGB4_EXT: Self = Self(0x804f);
    pub const GL_R11F_G11F_B10F_EXT: Self = Self(0x8c3a);
    pub const GL_COMPRESSED_RGBA_ASTC_4x3x3_OES: Self = Self(0x93c1);
    pub const GL_ALPHA16: Self = Self(0x803e);
    pub const GL_COMPRESSED_RG: Self = Self(0x8226);
    pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_12x12_KHR: Self = Self(0x93dd);
    pub const GL_R11F_G11F_B10F: Self = Self(0x8c3a);
    pub const GL_COMPRESSED_RGBA_ASTC_5x5: Self = Self(0x93b2);
    pub const GL_RGB5_A1_EXT: Self = Self(0x8057);
    pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_5x5_KHR: Self = Self(0x93d2);
    pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_5x4x4_OES: Self = Self(0x93e4);
    pub const GL_QUAD_ALPHA8_SGIS: Self = Self(0x811f);
    pub const GL_COMPRESSED_RGB: Self = Self(0x84ed);
    pub const GL_LUMINANCE4_ALPHA4_EXT: Self = Self(0x8043);
    pub const GL_ALPHA8_EXT: Self = Self(0x803c);
    pub const GL_RGBA16F: Self = Self(0x881a);
    pub const GL_LUMINANCE8UI_EXT: Self = Self(0x8d80);
    pub const GL_COMPRESSED_SIGNED_R11_EAC: Self = Self(0x9271);
    pub const GL_RGBA16UI: Self = Self(0x8d76);
    pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_5x4: Self = Self(0x93d1);
    pub const GL_COMPRESSED_RGB_BPTC_SIGNED_FLOAT: Self = Self(0x8e8e);
    pub const GL_LUMINANCE16: Self = Self(0x8042);
    pub const GL_DUAL_INTENSITY12_SGIS: Self = Self(0x811a);
    pub const GL_COMPRESSED_SRGB_ALPHA_S3TC_DXT3_EXT: Self = Self(0x8c4e);
    pub const GL_STENCIL_INDEX1_EXT: Self = Self(0x8d46);
    pub const GL_COMPRESSED_RGBA_ASTC_5x4: Self = Self(0x93b1);
    pub const GL_LUMINANCE_ALPHA32UI_EXT: Self = Self(0x8d75);
    pub const GL_RGBA16_SNORM_EXT: Self = Self(0x8f9b);
    pub const GL_LUMINANCE12_ALPHA4: Self = Self(0x8046);
    pub const GL_ALPHA32UI_EXT: Self = Self(0x8d72);
    pub const GL_ALPHA32I_EXT: Self = Self(0x8d84);
    pub const GL_RG16I: Self = Self(0x8239);
    pub const GL_SRGB: Self = Self(0x8c40);
    pub const GL_COMPRESSED_SIGNED_RED_RGTC1_EXT: Self = Self(0x8dbc);
    pub const GL_COMPRESSED_RGBA_S3TC_DXT5_ANGLE: Self = Self(0x83f3);
    pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_10x6: Self = Self(0x93d9);
    pub const GL_COMPRESSED_R11_EAC: Self = Self(0x9270);
    pub const GL_DUAL_LUMINANCE4_SGIS: Self = Self(0x8114);
    pub const GL_DUAL_ALPHA8_SGIS: Self = Self(0x8111);
    pub const GL_RGBA12: Self = Self(0x805a);
    pub const GL_COMPRESSED_RGB_S3TC_DXT1_EXT: Self = Self(0x83f0);
    pub const GL_COMPRESSED_SRGB_ALPHA_S3TC_DXT5_EXT: Self = Self(0x8c4f);
    pub const GL_RGB16I: Self = Self(0x8d89);
    pub const GL_RGB16I_EXT: Self = Self(0x8d89);
    pub const GL_RGBA8_OES: Self = Self(0x8058);
    pub const GL_RG16_SNORM_EXT: Self = Self(0x8f99);
    pub const GL_COMPRESSED_RGBA_ASTC_4x4: Self = Self(0x93b0);
    pub const GL_RGB8: Self = Self(0x8051);
    pub const GL_COMPRESSED_SIGNED_R11_EAC_OES: Self = Self(0x9271);
    pub const GL_RGB10_A2: Self = Self(0x8059);
    pub const GL_STENCIL_INDEX8: Self = Self(0x8d48);
    pub const GL_STENCIL_INDEX4: Self = Self(0x8d47);
    pub const GL_LUMINANCE32UI_EXT: Self = Self(0x8d74);
    pub const GL_RGB16F: Self = Self(0x881b);
    pub const GL_COMPRESSED_RED_RGTC1: Self = Self(0x8dbb);
    pub const GL_COMPRESSED_SRGB_ALPHA_BPTC_UNORM_ARB: Self = Self(0x8e8d);
    pub const GL_COMPRESSED_RGBA_ASTC_8x6: Self = Self(0x93b6);
    pub const GL_R8_SNORM: Self = Self(0x8f94);
    pub const GL_COMPRESSED_RGBA_ASTC_8x8_KHR: Self = Self(0x93b7);
    pub const GL_RGBA4: Self = Self(0x8056);
    pub const GL_ALPHA4_EXT: Self = Self(0x803b);
    pub const GL_DUAL_INTENSITY8_SGIS: Self = Self(0x8119);
    pub const GL_RGBA8_EXT: Self = Self(0x8058);
    pub const GL_STENCIL_INDEX16_EXT: Self = Self(0x8d49);
    pub const GL_RGB32UI: Self = Self(0x8d71);
    pub const GL_RGBA8_SNORM: Self = Self(0x8f97);
    pub const GL_COMPRESSED_RGBA_ASTC_4x4x3_OES: Self = Self(0x93c2);
    pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_4x4x4_OES: Self = Self(0x93e3);
    pub const GL_COMPRESSED_RGBA_BPTC_UNORM_EXT: Self = Self(0x8e8c);
    pub const GL_RGBA: Self = Self(0x1908);
    pub const GL_COMPRESSED_RGBA_ASTC_10x10_KHR: Self = Self(0x93bb);
    pub const GL_RGB32F_ARB: Self = Self(0x8815);
    pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_5x5x5_OES: Self = Self(0x93e6);
    pub const GL_COMPRESSED_RGBA_ASTC_5x4_KHR: Self = Self(0x93b1);
    pub const GL_DEPTH_STENCIL_OES: Self = Self(0x84f9);
    pub const GL_RG8_EXT: Self = Self(0x822b);
    pub const GL_R3_G3_B2: Self = Self(0x2a10);
    pub const GL_RGBA4_OES: Self = Self(0x8056);
    pub const GL_RGB565_OES: Self = Self(0x8d62);
    pub const GL_ALPHA12: Self = Self(0x803d);
    pub const GL_RED: Self = Self(0x1903);
    pub const GL_INTENSITY4: Self = Self(0x804a);
    pub const GL_INTENSITY16_EXT: Self = Self(0x804d);
    pub const GL_QUAD_LUMINANCE8_SGIS: Self = Self(0x8121);
    pub const GL_DEPTH_STENCIL_EXT: Self = Self(0x84f9);
    pub const GL_LUMINANCE4_ALPHA4: Self = Self(0x8043);
    pub const GL_STENCIL_INDEX16: Self = Self(0x8d49);
    pub const GL_RGB10_A2UI: Self = Self(0x906f);
    pub const GL_LUMINANCE16_EXT: Self = Self(0x8042);
    pub const GL_COMPRESSED_RGBA_ASTC_8x8: Self = Self(0x93b7);
    pub const GL_COMPRESSED_RGBA_ASTC_10x8_KHR: Self = Self(0x93ba);
    pub const GL_RGB8_SNORM: Self = Self(0x8f96);
    pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_4x3x3_OES: Self = Self(0x93e1);
    pub const GL_ALPHA8_OES: Self = Self(0x803c);
    pub const GL_RGB10_EXT: Self = Self(0x8052);
    pub const GL_LUMINANCE12_ALPHA12: Self = Self(0x8047);
    pub const GL_DEPTH_COMPONENT24: Self = Self(0x81a6);
    pub const GL_R8: Self = Self(0x8229);
    pub const GL_RG32UI: Self = Self(0x823c);
    pub const GL_LUMINANCE_ALPHA32I_EXT: Self = Self(0x8d87);
    pub const GL_RGB8I_EXT: Self = Self(0x8d8f);
    pub const GL_ALPHA8: Self = Self(0x803c);
    pub const GL_ALPHA16I_EXT: Self = Self(0x8d8a);
    pub const GL_STENCIL_INDEX: Self = Self(0x1901);
    pub const GL_RG16F_EXT: Self = Self(0x822f);
    pub const GL_LUMINANCE8I_EXT: Self = Self(0x8d92);
    pub const GL_DEPTH_COMPONENT16_OES: Self = Self(0x81a5);
    pub const GL_COMPRESSED_SIGNED_RG_RGTC2: Self = Self(0x8dbe);
    pub const GL_COMPRESSED_RGBA_BPTC_UNORM_ARB: Self = Self(0x8e8c);
    pub const GL_LUMINANCE16UI_EXT: Self = Self(0x8d7a);
    pub const GL_COMPRESSED_RED_RGTC1_EXT: Self = Self(0x8dbb);
    pub const GL_LUMINANCE8_EXT: Self = Self(0x8040);
    pub const GL_COMPRESSED_RGBA_ASTC_12x10_KHR: Self = Self(0x93bc);
    pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_4x4_KHR: Self = Self(0x93d0);
    pub const GL_COMPRESSED_SIGNED_RG11_EAC_OES: Self = Self(0x9273);
    pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_6x6x5_OES: Self = Self(0x93e8);
    pub const GL_RG8I: Self = Self(0x8237);
    pub const GL_DUAL_LUMINANCE16_SGIS: Self = Self(0x8117);
    pub const GL_RGB2_EXT: Self = Self(0x804e);
    pub const GL_SRGB8_ALPHA8: Self = Self(0x8c43);
    pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_8x6: Self = Self(0x93d6);
    pub const GL_DEPTH_COMPONENT24_OES: Self = Self(0x81a6);
    pub const GL_SRGB8_EXT: Self = Self(0x8c41);
    pub const GL_QUAD_INTENSITY4_SGIS: Self = Self(0x8122);
    pub const GL_DEPTH_COMPONENT32: Self = Self(0x81a7);
    pub const GL_DEPTH_STENCIL_NV: Self = Self(0x84f9);
    pub const GL_R32F_EXT: Self = Self(0x822e);
    pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_10x5_KHR: Self = Self(0x93d8);
    pub const GL_STENCIL_INDEX8_OES: Self = Self(0x8d48);
    pub const GL_RGBA16: Self = Self(0x805b);
    pub const GL_RGB10: Self = Self(0x8052);
    pub const GL_COMPRESSED_RG11_EAC_OES: Self = Self(0x9272);
    pub const GL_RGB12: Self = Self(0x8053);
    pub const GL_COMPRESSED_RGB_BPTC_SIGNED_FLOAT_ARB: Self = Self(0x8e8e);
    pub const GL_COMPRESSED_RGBA_ASTC_10x6: Self = Self(0x93b9);
    pub const GL_DUAL_LUMINANCE12_SGIS: Self = Self(0x8116);
    pub const GL_COMPRESSED_SRGB_ALPHA_S3TC_DXT1_EXT: Self = Self(0x8c4d);
    pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_12x10: Self = Self(0x93dc);
    pub const GL_COMPRESSED_R11_EAC_OES: Self = Self(0x9270);
    pub const GL_RGB16_EXT: Self = Self(0x8054);
    pub const GL_DUAL_INTENSITY16_SGIS: Self = Self(0x811b);
    pub const GL_INTENSITY8UI_EXT: Self = Self(0x8d7f);
    pub const GL_RGB5: Self = Self(0x8050);
    pub const GL_RGB9_E5_EXT: Self = Self(0x8c3d);
    pub const GL_RGB8UI_EXT: Self = Self(0x8d7d);
    pub const GL_RGB32I_EXT: Self = Self(0x8d83);
    pub const GL_INTENSITY32I_EXT: Self = Self(0x8d85);
    pub const GL_DUAL_LUMINANCE8_SGIS: Self = Self(0x8115);
    pub const GL_DEPTH_COMPONENT32_ARB: Self = Self(0x81a7);
    pub const GL_DUAL_ALPHA4_SGIS: Self = Self(0x8110);
    pub const GL_DEPTH_COMPONENT32_SGIX: Self = Self(0x81a7);
    pub const GL_R16F_EXT: Self = Self(0x822d);
    pub const GL_R16UI: Self = Self(0x8234);
    pub const GL_SRGB8: Self = Self(0x8c41);
    pub const GL_SRGB8_NV: Self = Self(0x8c41);
    pub const GL_COMPRESSED_RGB_BPTC_UNSIGNED_FLOAT_ARB: Self = Self(0x8e8f);
    pub const GL_INTENSITY16: Self = Self(0x804d);
    pub const GL_RGB10_A2_EXT: Self = Self(0x8059);
    pub const GL_RGBA32UI_EXT: Self = Self(0x8d70);
    pub const GL_INTENSITY32UI_EXT: Self = Self(0x8d73);
    pub const GL_ALPHA16_EXT: Self = Self(0x803e);
    pub const GL_RGBA8: Self = Self(0x8058);
    pub const GL_DEPTH_STENCIL: Self = Self(0x84f9);
    pub const GL_LUMINANCE6_ALPHA2_EXT: Self = Self(0x8044);
    pub const GL_RGBA16I_EXT: Self = Self(0x8d88);
    pub const GL_COMPRESSED_SRGB8_ETC2: Self = Self(0x9275);
    pub const GL_COMPRESSED_RGB8_PUNCHTHROUGH_ALPHA1_ETC2_OES: Self = Self(0x9276);
    pub const GL_R32I: Self = Self(0x8235);
    pub const GL_COMPRESSED_RGBA8_ETC2_EAC: Self = Self(0x9278);
    pub const GL_RGBA16F_ARB: Self = Self(0x881a);
    pub const GL_RGBA4_EXT: Self = Self(0x8056);
    pub const GL_COMPRESSED_RGBA_ASTC_6x5_KHR: Self = Self(0x93b3);
    pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_6x5_KHR: Self = Self(0x93d3);
    pub const GL_INTENSITY8_EXT: Self = Self(0x804b);
    pub const GL_INTENSITY: Self = Self(0x8049);
    pub const GL_COMPRESSED_RGB_BPTC_UNSIGNED_FLOAT: Self = Self(0x8e8f);
    pub const GL_COMPRESSED_SRGB8_ETC2_OES: Self = Self(0x9275);
    pub const GL_RGB32UI_EXT: Self = Self(0x8d71);
    pub const GL_LUMINANCE_ALPHA8UI_EXT: Self = Self(0x8d81);
    pub const GL_COMPRESSED_SRGB_ALPHA: Self = Self(0x8c49);
    pub const GL_R11F_G11F_B10F_APPLE: Self = Self(0x8c3a);
    pub const GL_RGBA8UI: Self = Self(0x8d7c);
    pub const GL_LUMINANCE_ALPHA16I_EXT: Self = Self(0x8d8d);
    pub const GL_COMPRESSED_SRGB_ALPHA_S3TC_DXT5_NV: Self = Self(0x8c4f);
    pub const GL_COMPRESSED_RGBA_ASTC_10x10: Self = Self(0x93bb);
    pub const GL_RGBA2_EXT: Self = Self(0x8055);
    pub const GL_COMPRESSED_RGBA_ASTC_6x5x5_OES: Self = Self(0x93c7);
    pub const GL_COMPRESSED_RGBA_ASTC_6x5: Self = Self(0x93b3);
    pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_5x5: Self = Self(0x93d2);
    pub const GL_LUMINANCE_ALPHA16UI_EXT: Self = Self(0x8d7b);
    pub const GL_R16I: Self = Self(0x8233);
    pub const GL_RGBA12_EXT: Self = Self(0x805a);
    pub const GL_DEPTH_STENCIL_MESA: Self = Self(0x8750);
    pub const GL_RGB9_E5: Self = Self(0x8c3d);
    pub const GL_RG8UI: Self = Self(0x8238);
    pub const GL_DEPTH24_STENCIL8_EXT: Self = Self(0x88f0);
    pub const GL_COMPRESSED_RGB_BPTC_UNSIGNED_FLOAT_EXT: Self = Self(0x8e8f);
    pub const GL_RGB16_SNORM: Self = Self(0x8f9a);
    pub const GL_QUAD_LUMINANCE4_SGIS: Self = Self(0x8120);
    pub const GL_RGB8I: Self = Self(0x8d8f);
    pub const GL_LUMINANCE6_ALPHA2: Self = Self(0x8044);
    pub const GL_COMPRESSED_RGBA_S3TC_DXT5_EXT: Self = Self(0x83f3);
    pub const GL_DUAL_LUMINANCE_ALPHA8_SGIS: Self = Self(0x811d);
    pub const GL_RG16: Self = Self(0x822c);
    pub const GL_COMPRESSED_RGB8_ETC2_OES: Self = Self(0x9274);
    pub const GL_LUMINANCE8: Self = Self(0x8040);
    pub const GL_SRGB_EXT: Self = Self(0x8c40);
    pub const GL_COMPRESSED_RGBA_ASTC_5x4x4_OES: Self = Self(0x93c4);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VertexShaderParameterEXT(pub(crate) u64);
impl VertexShaderParameterEXT {
    pub const GL_CURRENT_VERTEX_EXT: Self = Self(0x87e2);
    pub const GL_MVP_MATRIX_EXT: Self = Self(0x87e3);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VertexArrayPName(pub(crate) u64);
impl VertexArrayPName {
    pub const GL_VERTEX_ATTRIB_ARRAY_ENABLED: Self = Self(0x8622);
    pub const GL_VERTEX_ATTRIB_ARRAY_NORMALIZED: Self = Self(0x886a);
    pub const GL_VERTEX_ATTRIB_ARRAY_TYPE: Self = Self(0x8625);
    pub const GL_VERTEX_ATTRIB_ARRAY_INTEGER: Self = Self(0x88fd);
    pub const GL_VERTEX_ATTRIB_ARRAY_STRIDE: Self = Self(0x8624);
    pub const GL_VERTEX_ATTRIB_ARRAY_DIVISOR: Self = Self(0x88fe);
    pub const GL_VERTEX_ATTRIB_RELATIVE_OFFSET: Self = Self(0x82d5);
    pub const GL_VERTEX_ATTRIB_ARRAY_SIZE: Self = Self(0x8623);
    pub const GL_VERTEX_ATTRIB_ARRAY_LONG: Self = Self(0x874e);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ReadBufferMode(pub(crate) u64);
impl ReadBufferMode {
    pub const GL_COLOR_ATTACHMENT1: Self = Self(0x8ce1);
    pub const GL_COLOR_ATTACHMENT9: Self = Self(0x8ce9);
    pub const GL_COLOR_ATTACHMENT10: Self = Self(0x8cea);
    pub const GL_FRONT: Self = Self(0x0404);
    pub const GL_COLOR_ATTACHMENT7: Self = Self(0x8ce7);
    pub const GL_COLOR_ATTACHMENT0: Self = Self(0x8ce0);
    pub const GL_COLOR_ATTACHMENT4: Self = Self(0x8ce4);
    pub const GL_COLOR_ATTACHMENT15: Self = Self(0x8cef);
    pub const GL_COLOR_ATTACHMENT2: Self = Self(0x8ce2);
    pub const GL_COLOR_ATTACHMENT5: Self = Self(0x8ce5);
    pub const GL_COLOR_ATTACHMENT6: Self = Self(0x8ce6);
    pub const GL_COLOR_ATTACHMENT14: Self = Self(0x8cee);
    pub const GL_FRONT_LEFT: Self = Self(0x0400);
    pub const GL_RIGHT: Self = Self(0x0407);
    pub const GL_COLOR_ATTACHMENT13: Self = Self(0x8ced);
    pub const GL_BACK_LEFT: Self = Self(0x0402);
    pub const GL_NONE: Self = Self(0);
    pub const GL_FRONT_RIGHT: Self = Self(0x0401);
    pub const GL_AUX3: Self = Self(0x040c);
    pub const GL_COLOR_ATTACHMENT3: Self = Self(0x8ce3);
    pub const GL_AUX1: Self = Self(0x040a);
    pub const GL_COLOR_ATTACHMENT12: Self = Self(0x8cec);
    pub const GL_LEFT: Self = Self(0x0406);
    pub const GL_AUX0: Self = Self(0x0409);
    pub const GL_BACK_RIGHT: Self = Self(0x0403);
    pub const GL_NONE_OES: Self = Self(0);
    pub const GL_BACK: Self = Self(0x0405);
    pub const GL_COLOR_ATTACHMENT11: Self = Self(0x8ceb);
    pub const GL_COLOR_ATTACHMENT8: Self = Self(0x8ce8);
    pub const GL_AUX2: Self = Self(0x040b);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MatrixMode(pub(crate) u64);
impl MatrixMode {
    pub const GL_MODELVIEW0_EXT: Self = Self(0x1700);
    pub const GL_TEXTURE: Self = Self(0x1702);
    pub const GL_PROJECTION: Self = Self(0x1701);
    pub const GL_MODELVIEW: Self = Self(0x1700);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PrecisionType(pub(crate) u64);
impl PrecisionType {
    pub const GL_HIGH_INT: Self = Self(0x8df5);
    pub const GL_LOW_FLOAT: Self = Self(0x8df0);
    pub const GL_HIGH_FLOAT: Self = Self(0x8df2);
    pub const GL_LOW_INT: Self = Self(0x8df3);
    pub const GL_MEDIUM_FLOAT: Self = Self(0x8df1);
    pub const GL_MEDIUM_INT: Self = Self(0x8df4);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GetPointervPName(pub(crate) u64);
impl GetPointervPName {
    pub const GL_SELECTION_BUFFER_POINTER: Self = Self(0x0df3);
    pub const GL_TEXTURE_COORD_ARRAY_POINTER_EXT: Self = Self(0x8092);
    pub const GL_FEEDBACK_BUFFER_POINTER: Self = Self(0x0df0);
    pub const GL_EDGE_FLAG_ARRAY_POINTER: Self = Self(0x8093);
    pub const GL_EDGE_FLAG_ARRAY_POINTER_EXT: Self = Self(0x8093);
    pub const GL_INSTRUMENT_BUFFER_POINTER_SGIX: Self = Self(0x8180);
    pub const GL_TEXTURE_COORD_ARRAY_POINTER: Self = Self(0x8092);
    pub const GL_NORMAL_ARRAY_POINTER: Self = Self(0x808f);
    pub const GL_VERTEX_ARRAY_POINTER: Self = Self(0x808e);
    pub const GL_NORMAL_ARRAY_POINTER_EXT: Self = Self(0x808f);
    pub const GL_DEBUG_CALLBACK_USER_PARAM: Self = Self(0x8245);
    pub const GL_DEBUG_CALLBACK_FUNCTION: Self = Self(0x8244);
    pub const GL_VERTEX_ARRAY_POINTER_EXT: Self = Self(0x808e);
    pub const GL_COLOR_ARRAY_POINTER: Self = Self(0x8090);
    pub const GL_INDEX_ARRAY_POINTER: Self = Self(0x8091);
    pub const GL_COLOR_ARRAY_POINTER_EXT: Self = Self(0x8090);
    pub const GL_INDEX_ARRAY_POINTER_EXT: Self = Self(0x8091);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ElementPointerTypeATI(pub(crate) u64);
impl ElementPointerTypeATI {
    pub const GL_UNSIGNED_INT: Self = Self(0x1405);
    pub const GL_UNSIGNED_BYTE: Self = Self(0x1401);
    pub const GL_UNSIGNED_SHORT: Self = Self(0x1403);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VertexStreamATI(pub(crate) u64);
impl VertexStreamATI {
    pub const GL_VERTEX_STREAM2_ATI: Self = Self(0x876e);
    pub const GL_VERTEX_STREAM0_ATI: Self = Self(0x876c);
    pub const GL_VERTEX_STREAM7_ATI: Self = Self(0x8773);
    pub const GL_VERTEX_STREAM6_ATI: Self = Self(0x8772);
    pub const GL_VERTEX_STREAM1_ATI: Self = Self(0x876d);
    pub const GL_VERTEX_STREAM5_ATI: Self = Self(0x8771);
    pub const GL_VERTEX_STREAM3_ATI: Self = Self(0x876f);
    pub const GL_VERTEX_STREAM4_ATI: Self = Self(0x8770);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RenderbufferTarget(pub(crate) u64);
impl RenderbufferTarget {
    pub const GL_RENDERBUFFER: Self = Self(0x8d41);
    pub const GL_RENDERBUFFER_OES: Self = Self(0x8d41);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FogParameter(pub(crate) u64);
impl FogParameter {
    pub const GL_FOG_DENSITY: Self = Self(0x0b62);
    pub const GL_FOG_END: Self = Self(0x0b64);
    pub const GL_FOG_INDEX: Self = Self(0x0b61);
    pub const GL_FOG_OFFSET_VALUE_SGIX: Self = Self(0x8199);
    pub const GL_FOG_START: Self = Self(0x0b63);
    pub const GL_FOG_MODE: Self = Self(0x0b65);
    pub const GL_FOG_COLOR: Self = Self(0x0b66);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PixelStoreResampleMode(pub(crate) u64);
impl PixelStoreResampleMode {
    pub const GL_RESAMPLE_DECIMATE_SGIX: Self = Self(0x8430);
    pub const GL_RESAMPLE_REPLICATE_SGIX: Self = Self(0x8433);
    pub const GL_RESAMPLE_ZERO_FILL_SGIX: Self = Self(0x8434);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PreserveModeATI(pub(crate) u64);
impl PreserveModeATI {
    pub const GL_DISCARD_ATI: Self = Self(0x8763);
    pub const GL_PRESERVE_ATI: Self = Self(0x8762);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VertexShaderCoordOutEXT(pub(crate) u64);
impl VertexShaderCoordOutEXT {
    pub const GL_NEGATIVE_Y_EXT: Self = Self(0x87da);
    pub const GL_X_EXT: Self = Self(0x87d5);
    pub const GL_W_EXT: Self = Self(0x87d8);
    pub const GL_NEGATIVE_X_EXT: Self = Self(0x87d9);
    pub const GL_NEGATIVE_W_EXT: Self = Self(0x87dc);
    pub const GL_Z_EXT: Self = Self(0x87d7);
    pub const GL_ONE_EXT: Self = Self(0x87de);
    pub const GL_ZERO_EXT: Self = Self(0x87dd);
    pub const GL_Y_EXT: Self = Self(0x87d6);
    pub const GL_NEGATIVE_Z_EXT: Self = Self(0x87db);
    pub const GL_NEGATIVE_ONE_EXT: Self = Self(0x87df);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PNTrianglesPNameATI(pub(crate) u64);
impl PNTrianglesPNameATI {
    pub const GL_PN_TRIANGLES_NORMAL_MODE_ATI: Self = Self(0x87f3);
    pub const GL_PN_TRIANGLES_TESSELATION_LEVEL_ATI: Self = Self(0x87f4);
    pub const GL_PN_TRIANGLES_POINT_MODE_ATI: Self = Self(0x87f2);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PipelineParameterName(pub(crate) u64);
impl PipelineParameterName {
    pub const GL_VERTEX_SHADER: Self = Self(0x8b31);
    pub const GL_TESS_EVALUATION_SHADER: Self = Self(0x8e87);
    pub const GL_TESS_CONTROL_SHADER: Self = Self(0x8e88);
    pub const GL_GEOMETRY_SHADER: Self = Self(0x8dd9);
    pub const GL_FRAGMENT_SHADER: Self = Self(0x8b30);
    pub const GL_ACTIVE_PROGRAM: Self = Self(0x8259);
    pub const GL_INFO_LOG_LENGTH: Self = Self(0x8b84);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PathRenderingTokenNV(pub(crate) u64);
impl PathRenderingTokenNV {
    pub const GL_RELATIVE_MOVE_TO_NV: Self = Self(0x03);
    pub const GL_RELATIVE_SMALL_CCW_ARC_TO_NV: Self = Self(0x13);
    pub const GL_LARGE_CW_ARC_TO_NV: Self = Self(0x18);
    pub const GL_RELATIVE_ROUNDED_RECT_NV: Self = Self(0xe9);
    pub const GL_CUBIC_CURVE_TO_NV: Self = Self(0x0c);
    pub const GL_ARC_TO_NV: Self = Self(0xfe);
    pub const GL_RELATIVE_ROUNDED_RECT2_NV: Self = Self(0xeb);
    pub const GL_VERTICAL_LINE_TO_NV: Self = Self(0x08);
    pub const GL_SMOOTH_CUBIC_CURVE_TO_NV: Self = Self(0x10);
    pub const GL_RELATIVE_SMOOTH_CUBIC_CURVE_TO_NV: Self = Self(0x11);
    pub const GL_ROUNDED_RECT8_NV: Self = Self(0xee);
    pub const GL_RELATIVE_RECT_NV: Self = Self(0xf7);
    pub const GL_RELATIVE_ROUNDED_RECT8_NV: Self = Self(0xef);
    pub const GL_SMALL_CW_ARC_TO_NV: Self = Self(0x14);
    pub const GL_RELATIVE_LARGE_CCW_ARC_TO_NV: Self = Self(0x17);
    pub const GL_RELATIVE_ARC_TO_NV: Self = Self(0xff);
    pub const GL_CONIC_CURVE_TO_NV: Self = Self(0x1a);
    pub const GL_MOVE_TO_NV: Self = Self(0x02);
    pub const GL_RELATIVE_CUBIC_CURVE_TO_NV: Self = Self(0x0d);
    pub const GL_SHARED_EDGE_NV: Self = Self(0xc0);
    pub const GL_LINE_TO_NV: Self = Self(0x04);
    pub const GL_RELATIVE_QUADRATIC_CURVE_TO_NV: Self = Self(0x0b);
    pub const GL_QUADRATIC_CURVE_TO_NV: Self = Self(0x0a);
    pub const GL_RELATIVE_CONIC_CURVE_TO_NV: Self = Self(0x1b);
    pub const GL_CIRCULAR_CW_ARC_TO_NV: Self = Self(0xfa);
    pub const GL_SMOOTH_QUADRATIC_CURVE_TO_NV: Self = Self(0x0e);
    pub const GL_HORIZONTAL_LINE_TO_NV: Self = Self(0x06);
    pub const GL_SMALL_CCW_ARC_TO_NV: Self = Self(0x12);
    pub const GL_RESTART_PATH_NV: Self = Self(0xf0);
    pub const GL_ROUNDED_RECT2_NV: Self = Self(0xea);
    pub const GL_RELATIVE_VERTICAL_LINE_TO_NV: Self = Self(0x09);
    pub const GL_RELATIVE_HORIZONTAL_LINE_TO_NV: Self = Self(0x07);
    pub const GL_ROUNDED_RECT4_NV: Self = Self(0xec);
    pub const GL_RELATIVE_SMOOTH_QUADRATIC_CURVE_TO_NV: Self = Self(0x0f);
    pub const GL_RELATIVE_LARGE_CW_ARC_TO_NV: Self = Self(0x19);
    pub const GL_CIRCULAR_CCW_ARC_TO_NV: Self = Self(0xf8);
    pub const GL_RELATIVE_LINE_TO_NV: Self = Self(0x05);
    pub const GL_DUP_LAST_CUBIC_CURVE_TO_NV: Self = Self(0xf4);
    pub const GL_RELATIVE_SMALL_CW_ARC_TO_NV: Self = Self(0x15);
    pub const GL_RECT_NV: Self = Self(0xf6);
    pub const GL_LARGE_CCW_ARC_TO_NV: Self = Self(0x16);
    pub const GL_ROUNDED_RECT_NV: Self = Self(0xe8);
    pub const GL_CLOSE_PATH_NV: Self = Self(0x00);
    pub const GL_RELATIVE_ROUNDED_RECT4_NV: Self = Self(0xed);
    pub const GL_DUP_FIRST_CUBIC_CURVE_TO_NV: Self = Self(0xf2);
    pub const GL_CIRCULAR_TANGENT_ARC_TO_NV: Self = Self(0xfc);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ConvolutionTarget(pub(crate) u64);
impl ConvolutionTarget {
    pub const GL_CONVOLUTION_2D: Self = Self(0x8011);
    pub const GL_CONVOLUTION_1D: Self = Self(0x8010);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GetColorTableParameterPNameSGI(pub(crate) u64);
impl GetColorTableParameterPNameSGI {
    pub const GL_COLOR_TABLE_WIDTH: Self = Self(0x80d9);
    pub const GL_COLOR_TABLE_INTENSITY_SIZE: Self = Self(0x80df);
    pub const GL_COLOR_TABLE_RED_SIZE: Self = Self(0x80da);
    pub const GL_COLOR_TABLE_ALPHA_SIZE_SGI: Self = Self(0x80dd);
    pub const GL_COLOR_TABLE_SCALE: Self = Self(0x80d6);
    pub const GL_COLOR_TABLE_GREEN_SIZE: Self = Self(0x80db);
    pub const GL_COLOR_TABLE_GREEN_SIZE_SGI: Self = Self(0x80db);
    pub const GL_COLOR_TABLE_SCALE_SGI: Self = Self(0x80d6);
    pub const GL_COLOR_TABLE_ALPHA_SIZE: Self = Self(0x80dd);
    pub const GL_COLOR_TABLE_BIAS: Self = Self(0x80d7);
    pub const GL_COLOR_TABLE_INTENSITY_SIZE_SGI: Self = Self(0x80df);
    pub const GL_COLOR_TABLE_WIDTH_SGI: Self = Self(0x80d9);
    pub const GL_COLOR_TABLE_LUMINANCE_SIZE_SGI: Self = Self(0x80de);
    pub const GL_COLOR_TABLE_FORMAT: Self = Self(0x80d8);
    pub const GL_COLOR_TABLE_FORMAT_SGI: Self = Self(0x80d8);
    pub const GL_COLOR_TABLE_BIAS_SGI: Self = Self(0x80d7);
    pub const GL_COLOR_TABLE_BLUE_SIZE_SGI: Self = Self(0x80dc);
    pub const GL_COLOR_TABLE_RED_SIZE_SGI: Self = Self(0x80da);
    pub const GL_COLOR_TABLE_BLUE_SIZE: Self = Self(0x80dc);
    pub const GL_COLOR_TABLE_LUMINANCE_SIZE: Self = Self(0x80de);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StringName(pub(crate) u64);
impl StringName {
    pub const GL_SHADING_LANGUAGE_VERSION: Self = Self(0x8b8c);
    pub const GL_EXTENSIONS: Self = Self(0x1f03);
    pub const GL_VENDOR: Self = Self(0x1f00);
    pub const GL_VERSION: Self = Self(0x1f02);
    pub const GL_RENDERER: Self = Self(0x1f01);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LightModelParameter(pub(crate) u64);
impl LightModelParameter {
    pub const GL_LIGHT_MODEL_COLOR_CONTROL: Self = Self(0x81f8);
    pub const GL_LIGHT_MODEL_TWO_SIDE: Self = Self(0x0b52);
    pub const GL_LIGHT_MODEL_COLOR_CONTROL_EXT: Self = Self(0x81f8);
    pub const GL_LIGHT_MODEL_LOCAL_VIEWER: Self = Self(0x0b51);
    pub const GL_LIGHT_MODEL_AMBIENT: Self = Self(0x0b53);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VertexShaderWriteMaskEXT(pub(crate) u64);
impl VertexShaderWriteMaskEXT {
    pub const GL_FALSE: Self = Self(0);
    pub const GL_TRUE: Self = Self(1);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SemaphoreParameterName(pub(crate) u64);
impl SemaphoreParameterName {
    pub const GL_TIMELINE_SEMAPHORE_VALUE_NV: Self = Self(0x9595);
    pub const GL_D3D12_FENCE_VALUE_EXT: Self = Self(0x9595);
    pub const GL_SEMAPHORE_TYPE_NV: Self = Self(0x95b3);
    pub const GL_SEMAPHORE_TYPE_BINARY_NV: Self = Self(0x95b4);
    pub const GL_SEMAPHORE_TYPE_TIMELINE_NV: Self = Self(0x95b5);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MinmaxTarget(pub(crate) u64);
impl MinmaxTarget {
    pub const GL_MINMAX: Self = Self(0x802e);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TexStorageAttribs(pub(crate) u64);
impl TexStorageAttribs {
    pub const GL_SURFACE_COMPRESSION_FIXED_RATE_2BPC_EXT: Self = Self(0x96c5);
    pub const GL_SURFACE_COMPRESSION_FIXED_RATE_11BPC_EXT: Self = Self(0x96ce);
    pub const GL_SURFACE_COMPRESSION_FIXED_RATE_12BPC_EXT: Self = Self(0x96cf);
    pub const GL_SURFACE_COMPRESSION_EXT: Self = Self(0x96c0);
    pub const GL_SURFACE_COMPRESSION_FIXED_RATE_1BPC_EXT: Self = Self(0x96c4);
    pub const GL_SURFACE_COMPRESSION_FIXED_RATE_10BPC_EXT: Self = Self(0x96cd);
    pub const GL_SURFACE_COMPRESSION_FIXED_RATE_DEFAULT_EXT: Self = Self(0x96c2);
    pub const GL_SURFACE_COMPRESSION_FIXED_RATE_5BPC_EXT: Self = Self(0x96c8);
    pub const GL_SURFACE_COMPRESSION_FIXED_RATE_7BPC_EXT: Self = Self(0x96ca);
    pub const GL_SURFACE_COMPRESSION_FIXED_RATE_8BPC_EXT: Self = Self(0x96cb);
    pub const GL_SURFACE_COMPRESSION_FIXED_RATE_6BPC_EXT: Self = Self(0x96c9);
    pub const GL_SURFACE_COMPRESSION_FIXED_RATE_9BPC_EXT: Self = Self(0x96cc);
    pub const GL_SURFACE_COMPRESSION_FIXED_RATE_3BPC_EXT: Self = Self(0x96c6);
    pub const GL_SURFACE_COMPRESSION_FIXED_RATE_4BPC_EXT: Self = Self(0x96c7);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ColorBuffer(pub(crate) u64);
impl ColorBuffer {
    pub const GL_NONE: Self = Self(0);
    pub const GL_FRONT: Self = Self(0x0404);
    pub const GL_RIGHT: Self = Self(0x0407);
    pub const GL_COLOR_ATTACHMENT3: Self = Self(0x8ce3);
    pub const GL_COLOR_ATTACHMENT5: Self = Self(0x8ce5);
    pub const GL_BACK_RIGHT: Self = Self(0x0403);
    pub const GL_COLOR_ATTACHMENT15: Self = Self(0x8cef);
    pub const GL_FRONT_RIGHT: Self = Self(0x0401);
    pub const GL_COLOR_ATTACHMENT19: Self = Self(0x8cf3);
    pub const GL_COLOR_ATTACHMENT23: Self = Self(0x8cf7);
    pub const GL_COLOR_ATTACHMENT7: Self = Self(0x8ce7);
    pub const GL_COLOR_ATTACHMENT29: Self = Self(0x8cfd);
    pub const GL_COLOR_ATTACHMENT4: Self = Self(0x8ce4);
    pub const GL_COLOR_ATTACHMENT17: Self = Self(0x8cf1);
    pub const GL_BACK: Self = Self(0x0405);
    pub const GL_FRONT_AND_BACK: Self = Self(0x0408);
    pub const GL_COLOR_ATTACHMENT30: Self = Self(0x8cfe);
    pub const GL_LEFT: Self = Self(0x0406);
    pub const GL_COLOR_ATTACHMENT14: Self = Self(0x8cee);
    pub const GL_COLOR_ATTACHMENT2: Self = Self(0x8ce2);
    pub const GL_BACK_LEFT: Self = Self(0x0402);
    pub const GL_COLOR_ATTACHMENT10: Self = Self(0x8cea);
    pub const GL_COLOR_ATTACHMENT21: Self = Self(0x8cf5);
    pub const GL_COLOR_ATTACHMENT28: Self = Self(0x8cfc);
    pub const GL_COLOR_ATTACHMENT9: Self = Self(0x8ce9);
    pub const GL_COLOR_ATTACHMENT31: Self = Self(0x8cff);
    pub const GL_COLOR_ATTACHMENT13: Self = Self(0x8ced);
    pub const GL_COLOR_ATTACHMENT25: Self = Self(0x8cf9);
    pub const GL_COLOR_ATTACHMENT8: Self = Self(0x8ce8);
    pub const GL_COLOR_ATTACHMENT26: Self = Self(0x8cfa);
    pub const GL_COLOR_ATTACHMENT16: Self = Self(0x8cf0);
    pub const GL_COLOR_ATTACHMENT22: Self = Self(0x8cf6);
    pub const GL_COLOR_ATTACHMENT11: Self = Self(0x8ceb);
    pub const GL_COLOR_ATTACHMENT18: Self = Self(0x8cf2);
    pub const GL_COLOR_ATTACHMENT24: Self = Self(0x8cf8);
    pub const GL_COLOR_ATTACHMENT12: Self = Self(0x8cec);
    pub const GL_COLOR_ATTACHMENT0: Self = Self(0x8ce0);
    pub const GL_FRONT_LEFT: Self = Self(0x0400);
    pub const GL_COLOR_ATTACHMENT20: Self = Self(0x8cf4);
    pub const GL_COLOR_ATTACHMENT6: Self = Self(0x8ce6);
    pub const GL_COLOR_ATTACHMENT27: Self = Self(0x8cfb);
    pub const GL_COLOR_ATTACHMENT1: Self = Self(0x8ce1);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PathFontStyle(pub(crate) u64);
impl PathFontStyle {
    pub const GL_NONE: Self = Self(0);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CopyBufferSubDataTarget(pub(crate) u64);
impl CopyBufferSubDataTarget {
    pub const GL_ELEMENT_ARRAY_BUFFER: Self = Self(0x8893);
    pub const GL_PIXEL_UNPACK_BUFFER: Self = Self(0x88ec);
    pub const GL_SHADER_STORAGE_BUFFER: Self = Self(0x90d2);
    pub const GL_ARRAY_BUFFER: Self = Self(0x8892);
    pub const GL_TEXTURE_BUFFER: Self = Self(0x8c2a);
    pub const GL_QUERY_BUFFER: Self = Self(0x9192);
    pub const GL_COPY_READ_BUFFER: Self = Self(0x8f36);
    pub const GL_DRAW_INDIRECT_BUFFER: Self = Self(0x8f3f);
    pub const GL_ATOMIC_COUNTER_BUFFER: Self = Self(0x92c0);
    pub const GL_PIXEL_PACK_BUFFER: Self = Self(0x88eb);
    pub const GL_TRANSFORM_FEEDBACK_BUFFER: Self = Self(0x8c8e);
    pub const GL_COPY_WRITE_BUFFER: Self = Self(0x8f37);
    pub const GL_DISPATCH_INDIRECT_BUFFER: Self = Self(0x90ee);
    pub const GL_UNIFORM_BUFFER: Self = Self(0x8a11);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct QueryCounterTarget(pub(crate) u64);
impl QueryCounterTarget {
    pub const GL_TIMESTAMP: Self = Self(0x8e28);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PrimitiveType(pub(crate) u64);
impl PrimitiveType {
    pub const GL_LINES_ADJACENCY: Self = Self(0x000a);
    pub const GL_LINES_ADJACENCY_EXT: Self = Self(0x000a);
    pub const GL_PATCHES: Self = Self(0x000e);
    pub const GL_TRIANGLE_FAN: Self = Self(0x0006);
    pub const GL_TRIANGLES: Self = Self(0x0004);
    pub const GL_TRIANGLE_STRIP_ADJACENCY_ARB: Self = Self(0x000d);
    pub const GL_LINE_STRIP: Self = Self(0x0003);
    pub const GL_PATCHES_EXT: Self = Self(0x000e);
    pub const GL_TRIANGLE_STRIP: Self = Self(0x0005);
    pub const GL_POINTS: Self = Self(0x0000);
    pub const GL_LINE_STRIP_ADJACENCY_ARB: Self = Self(0x000b);
    pub const GL_TRIANGLE_STRIP_ADJACENCY: Self = Self(0x000d);
    pub const GL_TRIANGLE_STRIP_ADJACENCY_EXT: Self = Self(0x000d);
    pub const GL_QUADS: Self = Self(0x0007);
    pub const GL_QUAD_STRIP: Self = Self(0x0008);
    pub const GL_LINES: Self = Self(0x0001);
    pub const GL_LINE_LOOP: Self = Self(0x0002);
    pub const GL_QUADS_EXT: Self = Self(0x0007);
    pub const GL_POLYGON: Self = Self(0x0009);
    pub const GL_TRIANGLES_ADJACENCY_ARB: Self = Self(0x000c);
    pub const GL_LINES_ADJACENCY_ARB: Self = Self(0x000a);
    pub const GL_TRIANGLES_ADJACENCY_EXT: Self = Self(0x000c);
    pub const GL_LINE_STRIP_ADJACENCY_EXT: Self = Self(0x000b);
    pub const GL_TRIANGLES_ADJACENCY: Self = Self(0x000c);
    pub const GL_LINE_STRIP_ADJACENCY: Self = Self(0x000b);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LightParameter(pub(crate) u64);
impl LightParameter {
    pub const GL_QUADRATIC_ATTENUATION: Self = Self(0x1209);
    pub const GL_SPOT_CUTOFF: Self = Self(0x1206);
    pub const GL_SPOT_EXPONENT: Self = Self(0x1205);
    pub const GL_POSITION: Self = Self(0x1203);
    pub const GL_SPOT_DIRECTION: Self = Self(0x1204);
    pub const GL_CONSTANT_ATTENUATION: Self = Self(0x1207);
    pub const GL_LINEAR_ATTENUATION: Self = Self(0x1208);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TextureEnvParameter(pub(crate) u64);
impl TextureEnvParameter {
    pub const GL_TEXTURE_ENV_COLOR: Self = Self(0x2201);
    pub const GL_TEXTURE_ENV_MODE: Self = Self(0x2200);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VertexArrayPNameAPPLE(pub(crate) u64);
impl VertexArrayPNameAPPLE {
    pub const GL_STORAGE_SHARED_APPLE: Self = Self(0x85bf);
    pub const GL_STORAGE_CLIENT_APPLE: Self = Self(0x85b4);
    pub const GL_STORAGE_CACHED_APPLE: Self = Self(0x85be);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ProgramInterface(pub(crate) u64);
impl ProgramInterface {
    pub const GL_PROGRAM_INPUT: Self = Self(0x92e3);
    pub const GL_TESS_CONTROL_SUBROUTINE_UNIFORM: Self = Self(0x92ef);
    pub const GL_PROGRAM_OUTPUT: Self = Self(0x92e4);
    pub const GL_TRANSFORM_FEEDBACK_BUFFER: Self = Self(0x8c8e);
    pub const GL_FRAGMENT_SUBROUTINE: Self = Self(0x92ec);
    pub const GL_COMPUTE_SUBROUTINE: Self = Self(0x92ed);
    pub const GL_SHADER_STORAGE_BLOCK: Self = Self(0x92e6);
    pub const GL_TESS_CONTROL_SUBROUTINE: Self = Self(0x92e9);
    pub const GL_VERTEX_SUBROUTINE: Self = Self(0x92e8);
    pub const GL_UNIFORM: Self = Self(0x92e1);
    pub const GL_UNIFORM_BLOCK: Self = Self(0x92e2);
    pub const GL_BUFFER_VARIABLE: Self = Self(0x92e5);
    pub const GL_TESS_EVALUATION_SUBROUTINE: Self = Self(0x92ea);
    pub const GL_FRAGMENT_SUBROUTINE_UNIFORM: Self = Self(0x92f2);
    pub const GL_COMPUTE_SUBROUTINE_UNIFORM: Self = Self(0x92f3);
    pub const GL_TESS_EVALUATION_SUBROUTINE_UNIFORM: Self = Self(0x92f0);
    pub const GL_GEOMETRY_SUBROUTINE: Self = Self(0x92eb);
    pub const GL_GEOMETRY_SUBROUTINE_UNIFORM: Self = Self(0x92f1);
    pub const GL_TRANSFORM_FEEDBACK_VARYING: Self = Self(0x92f4);
    pub const GL_VERTEX_SUBROUTINE_UNIFORM: Self = Self(0x92ee);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ColorTableTarget(pub(crate) u64);
impl ColorTableTarget {
    pub const GL_COLOR_TABLE: Self = Self(0x80d0);
    pub const GL_PROXY_COLOR_TABLE: Self = Self(0x80d3);
    pub const GL_PROXY_POST_COLOR_MATRIX_COLOR_TABLE: Self = Self(0x80d5);
    pub const GL_POST_CONVOLUTION_COLOR_TABLE: Self = Self(0x80d1);
    pub const GL_PROXY_POST_CONVOLUTION_COLOR_TABLE: Self = Self(0x80d4);
    pub const GL_POST_COLOR_MATRIX_COLOR_TABLE: Self = Self(0x80d2);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FfdTargetSGIX(pub(crate) u64);
impl FfdTargetSGIX {
    pub const GL_GEOMETRY_DEFORMATION_SGIX: Self = Self(0x8194);
    pub const GL_TEXTURE_DEFORMATION_SGIX: Self = Self(0x8195);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ProgramFormat(pub(crate) u64);
impl ProgramFormat {
    pub const GL_PROGRAM_FORMAT_ASCII_ARB: Self = Self(0x8875);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ClipPlaneName(pub(crate) u64);
impl ClipPlaneName {
    pub const GL_CLIP_DISTANCE5: Self = Self(0x3005);
    pub const GL_CLIP_DISTANCE7: Self = Self(0x3007);
    pub const GL_CLIP_DISTANCE1: Self = Self(0x3001);
    pub const GL_CLIP_PLANE4: Self = Self(0x3004);
    pub const GL_CLIP_PLANE2: Self = Self(0x3002);
    pub const GL_CLIP_DISTANCE6: Self = Self(0x3006);
    pub const GL_CLIP_DISTANCE0: Self = Self(0x3000);
    pub const GL_CLIP_PLANE0: Self = Self(0x3000);
    pub const GL_CLIP_DISTANCE2: Self = Self(0x3002);
    pub const GL_CLIP_PLANE1: Self = Self(0x3001);
    pub const GL_CLIP_DISTANCE3: Self = Self(0x3003);
    pub const GL_CLIP_PLANE3: Self = Self(0x3003);
    pub const GL_CLIP_DISTANCE4: Self = Self(0x3004);
    pub const GL_CLIP_PLANE5: Self = Self(0x3005);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ConvolutionBorderModeEXT(pub(crate) u64);
impl ConvolutionBorderModeEXT {
    pub const GL_REDUCE_EXT: Self = Self(0x8016);
    pub const GL_REDUCE: Self = Self(0x8016);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ClipControlOrigin(pub(crate) u64);
impl ClipControlOrigin {
    pub const GL_LOWER_LEFT: Self = Self(0x8ca1);
    pub const GL_UPPER_LEFT: Self = Self(0x8ca2);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RenderbufferParameterName(pub(crate) u64);
impl RenderbufferParameterName {
    pub const GL_RENDERBUFFER_STENCIL_SIZE_EXT: Self = Self(0x8d55);
    pub const GL_RENDERBUFFER_INTERNAL_FORMAT_EXT: Self = Self(0x8d44);
    pub const GL_RENDERBUFFER_INTERNAL_FORMAT_OES: Self = Self(0x8d44);
    pub const GL_RENDERBUFFER_WIDTH_EXT: Self = Self(0x8d42);
    pub const GL_RENDERBUFFER_HEIGHT_OES: Self = Self(0x8d43);
    pub const GL_RENDERBUFFER_DEPTH_SIZE: Self = Self(0x8d54);
    pub const GL_RENDERBUFFER_HEIGHT_EXT: Self = Self(0x8d43);
    pub const GL_RENDERBUFFER_HEIGHT: Self = Self(0x8d43);
    pub const GL_RENDERBUFFER_RED_SIZE_EXT: Self = Self(0x8d50);
    pub const GL_RENDERBUFFER_SAMPLES: Self = Self(0x8cab);
    pub const GL_RENDERBUFFER_ALPHA_SIZE: Self = Self(0x8d53);
    pub const GL_RENDERBUFFER_STENCIL_SIZE: Self = Self(0x8d55);
    pub const GL_RENDERBUFFER_DEPTH_SIZE_OES: Self = Self(0x8d54);
    pub const GL_RENDERBUFFER_STENCIL_SIZE_OES: Self = Self(0x8d55);
    pub const GL_RENDERBUFFER_COVERAGE_SAMPLES_NV: Self = Self(0x8cab);
    pub const GL_RENDERBUFFER_BLUE_SIZE_OES: Self = Self(0x8d52);
    pub const GL_RENDERBUFFER_SAMPLES_NV: Self = Self(0x8cab);
    pub const GL_RENDERBUFFER_BLUE_SIZE: Self = Self(0x8d52);
    pub const GL_RENDERBUFFER_SAMPLES_IMG: Self = Self(0x9133);
    pub const GL_RENDERBUFFER_SAMPLES_APPLE: Self = Self(0x8cab);
    pub const GL_RENDERBUFFER_DEPTH_SIZE_EXT: Self = Self(0x8d54);
    pub const GL_RENDERBUFFER_GREEN_SIZE_OES: Self = Self(0x8d51);
    pub const GL_RENDERBUFFER_INTERNAL_FORMAT: Self = Self(0x8d44);
    pub const GL_RENDERBUFFER_COLOR_SAMPLES_NV: Self = Self(0x8e10);
    pub const GL_RENDERBUFFER_RED_SIZE_OES: Self = Self(0x8d50);
    pub const GL_RENDERBUFFER_SAMPLES_ANGLE: Self = Self(0x8cab);
    pub const GL_RENDERBUFFER_GREEN_SIZE_EXT: Self = Self(0x8d51);
    pub const GL_RENDERBUFFER_STORAGE_SAMPLES_AMD: Self = Self(0x91b2);
    pub const GL_RENDERBUFFER_RED_SIZE: Self = Self(0x8d50);
    pub const GL_RENDERBUFFER_WIDTH: Self = Self(0x8d42);
    pub const GL_RENDERBUFFER_SAMPLES_EXT: Self = Self(0x8cab);
    pub const GL_RENDERBUFFER_ALPHA_SIZE_OES: Self = Self(0x8d53);
    pub const GL_RENDERBUFFER_GREEN_SIZE: Self = Self(0x8d51);
    pub const GL_RENDERBUFFER_BLUE_SIZE_EXT: Self = Self(0x8d52);
    pub const GL_RENDERBUFFER_WIDTH_OES: Self = Self(0x8d42);
    pub const GL_RENDERBUFFER_ALPHA_SIZE_EXT: Self = Self(0x8d53);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BindTransformFeedbackTarget(pub(crate) u64);
impl BindTransformFeedbackTarget {
    pub const GL_TRANSFORM_FEEDBACK: Self = Self(0x8e22);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PathHandleMissingGlyphs(pub(crate) u64);
impl PathHandleMissingGlyphs {
    pub const GL_SKIP_MISSING_GLYPH_NV: Self = Self(0x90a9);
    pub const GL_USE_MISSING_GLYPH_NV: Self = Self(0x90aa);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TextureTarget(pub(crate) u64);
impl TextureTarget {
    pub const GL_TEXTURE_CUBE_MAP_NEGATIVE_X: Self = Self(0x8516);
    pub const GL_PROXY_TEXTURE_2D_ARRAY: Self = Self(0x8c1b);
    pub const GL_TEXTURE_CUBE_MAP: Self = Self(0x8513);
    pub const GL_TEXTURE_CUBE_MAP_OES: Self = Self(0x8513);
    pub const GL_PROXY_TEXTURE_1D: Self = Self(0x8063);
    pub const GL_PROXY_TEXTURE_CUBE_MAP: Self = Self(0x851b);
    pub const GL_PROXY_TEXTURE_CUBE_MAP_ARB: Self = Self(0x851b);
    pub const GL_TEXTURE_BUFFER: Self = Self(0x8c2a);
    pub const GL_TEXTURE_1D: Self = Self(0x0de0);
    pub const GL_PROXY_TEXTURE_3D: Self = Self(0x8070);
    pub const GL_TEXTURE_CUBE_MAP_ARRAY: Self = Self(0x9009);
    pub const GL_TEXTURE_3D: Self = Self(0x806f);
    pub const GL_PROXY_TEXTURE_2D: Self = Self(0x8064);
    pub const GL_TEXTURE_4D_SGIS: Self = Self(0x8134);
    pub const GL_TEXTURE_3D_EXT: Self = Self(0x806f);
    pub const GL_PROXY_TEXTURE_RECTANGLE_NV: Self = Self(0x84f7);
    pub const GL_TEXTURE_2D_MULTISAMPLE: Self = Self(0x9100);
    pub const GL_TEXTURE_2D_MULTISAMPLE_ARRAY: Self = Self(0x9102);
    pub const GL_DETAIL_TEXTURE_2D_SGIS: Self = Self(0x8095);
    pub const GL_PROXY_TEXTURE_CUBE_MAP_ARRAY: Self = Self(0x900b);
    pub const GL_TEXTURE_2D: Self = Self(0x0de1);
    pub const GL_TEXTURE_3D_OES: Self = Self(0x806f);
    pub const GL_TEXTURE_CUBE_MAP_POSITIVE_Z: Self = Self(0x8519);
    pub const GL_PROXY_TEXTURE_2D_EXT: Self = Self(0x8064);
    pub const GL_PROXY_TEXTURE_2D_ARRAY_EXT: Self = Self(0x8c1b);
    pub const GL_TEXTURE_RECTANGLE_ARB: Self = Self(0x84f5);
    pub const GL_TEXTURE_RECTANGLE_NV: Self = Self(0x84f5);
    pub const GL_TEXTURE_CUBE_MAP_POSITIVE_X: Self = Self(0x8515);
    pub const GL_TEXTURE_CUBE_MAP_ARRAY_OES: Self = Self(0x9009);
    pub const GL_PROXY_TEXTURE_4D_SGIS: Self = Self(0x8135);
    pub const GL_TEXTURE_CUBE_MAP_POSITIVE_Y: Self = Self(0x8517);
    pub const GL_PROXY_TEXTURE_2D_MULTISAMPLE: Self = Self(0x9101);
    pub const GL_PROXY_TEXTURE_CUBE_MAP_EXT: Self = Self(0x851b);
    pub const GL_TEXTURE_CUBE_MAP_NEGATIVE_Z: Self = Self(0x851a);
    pub const GL_TEXTURE_CUBE_MAP_ARRAY_EXT: Self = Self(0x9009);
    pub const GL_PROXY_TEXTURE_3D_EXT: Self = Self(0x8070);
    pub const GL_PROXY_TEXTURE_1D_ARRAY_EXT: Self = Self(0x8c19);
    pub const GL_RENDERBUFFER: Self = Self(0x8d41);
    pub const GL_TEXTURE_2D_ARRAY: Self = Self(0x8c1a);
    pub const GL_PROXY_TEXTURE_CUBE_MAP_ARRAY_ARB: Self = Self(0x900b);
    pub const GL_PROXY_TEXTURE_1D_EXT: Self = Self(0x8063);
    pub const GL_TEXTURE_RECTANGLE: Self = Self(0x84f5);
    pub const GL_PROXY_TEXTURE_RECTANGLE: Self = Self(0x84f7);
    pub const GL_PROXY_TEXTURE_RECTANGLE_ARB: Self = Self(0x84f7);
    pub const GL_PROXY_TEXTURE_1D_ARRAY: Self = Self(0x8c19);
    pub const GL_TEXTURE_CUBE_MAP_ARRAY_ARB: Self = Self(0x9009);
    pub const GL_TEXTURE_CUBE_MAP_ARB: Self = Self(0x8513);
    pub const GL_TEXTURE_CUBE_MAP_NEGATIVE_Y: Self = Self(0x8518);
    pub const GL_TEXTURE_1D_ARRAY: Self = Self(0x8c18);
    pub const GL_TEXTURE_CUBE_MAP_EXT: Self = Self(0x8513);
    pub const GL_PROXY_TEXTURE_2D_MULTISAMPLE_ARRAY: Self = Self(0x9103);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct HistogramTarget(pub(crate) u64);
impl HistogramTarget {
    pub const GL_HISTOGRAM: Self = Self(0x8024);
    pub const GL_PROXY_HISTOGRAM: Self = Self(0x8025);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ProgramStagePName(pub(crate) u64);
impl ProgramStagePName {
    pub const GL_ACTIVE_SUBROUTINES: Self = Self(0x8de5);
    pub const GL_ACTIVE_SUBROUTINE_UNIFORMS: Self = Self(0x8de6);
    pub const GL_ACTIVE_SUBROUTINE_MAX_LENGTH: Self = Self(0x8e48);
    pub const GL_ACTIVE_SUBROUTINE_UNIFORM_MAX_LENGTH: Self = Self(0x8e49);
    pub const GL_ACTIVE_SUBROUTINE_UNIFORM_LOCATIONS: Self = Self(0x8e47);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ColorTableTargetSGI(pub(crate) u64);
impl ColorTableTargetSGI {
    pub const GL_COLOR_TABLE: Self = Self(0x80d0);
    pub const GL_POST_COLOR_MATRIX_COLOR_TABLE: Self = Self(0x80d2);
    pub const GL_POST_CONVOLUTION_COLOR_TABLE: Self = Self(0x80d1);
    pub const GL_COLOR_TABLE_SGI: Self = Self(0x80d0);
    pub const GL_PROXY_POST_CONVOLUTION_COLOR_TABLE: Self = Self(0x80d4);
    pub const GL_POST_COLOR_MATRIX_COLOR_TABLE_SGI: Self = Self(0x80d2);
    pub const GL_PROXY_COLOR_TABLE: Self = Self(0x80d3);
    pub const GL_PROXY_POST_CONVOLUTION_COLOR_TABLE_SGI: Self = Self(0x80d4);
    pub const GL_PROXY_TEXTURE_COLOR_TABLE_SGI: Self = Self(0x80bd);
    pub const GL_POST_CONVOLUTION_COLOR_TABLE_SGI: Self = Self(0x80d1);
    pub const GL_TEXTURE_COLOR_TABLE_SGI: Self = Self(0x80bc);
    pub const GL_PROXY_COLOR_TABLE_SGI: Self = Self(0x80d3);
    pub const GL_PROXY_POST_COLOR_MATRIX_COLOR_TABLE: Self = Self(0x80d5);
    pub const GL_PROXY_POST_COLOR_MATRIX_COLOR_TABLE_SGI: Self = Self(0x80d5);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BufferPNameARB(pub(crate) u64);
impl BufferPNameARB {
    pub const GL_BUFFER_IMMUTABLE_STORAGE: Self = Self(0x821f);
    pub const GL_BUFFER_STORAGE_FLAGS: Self = Self(0x8220);
    pub const GL_BUFFER_ACCESS_ARB: Self = Self(0x88bb);
    pub const GL_BUFFER_MAPPED: Self = Self(0x88bc);
    pub const GL_BUFFER_MAPPED_ARB: Self = Self(0x88bc);
    pub const GL_BUFFER_MAP_LENGTH: Self = Self(0x9120);
    pub const GL_BUFFER_MAP_OFFSET: Self = Self(0x9121);
    pub const GL_BUFFER_ACCESS_FLAGS: Self = Self(0x911f);
    pub const GL_BUFFER_ACCESS: Self = Self(0x88bb);
    pub const GL_BUFFER_USAGE_ARB: Self = Self(0x8765);
    pub const GL_BUFFER_SIZE: Self = Self(0x8764);
    pub const GL_BUFFER_SIZE_ARB: Self = Self(0x8764);
    pub const GL_BUFFER_USAGE: Self = Self(0x8765);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FramebufferAttachmentParameterName(pub(crate) u64);
impl FramebufferAttachmentParameterName {
    pub const GL_FRAMEBUFFER_ATTACHMENT_RED_SIZE: Self = Self(0x8212);
    pub const GL_FRAMEBUFFER_ATTACHMENT_TEXTURE_LAYER: Self = Self(0x8cd4);
    pub const GL_FRAMEBUFFER_ATTACHMENT_TEXTURE_SAMPLES_EXT: Self = Self(0x8d6c);
    pub const GL_FRAMEBUFFER_ATTACHMENT_TEXTURE_3D_ZOFFSET_OES: Self = Self(0x8cd4);
    pub const GL_FRAMEBUFFER_ATTACHMENT_LAYERED_ARB: Self = Self(0x8da7);
    pub const GL_FRAMEBUFFER_ATTACHMENT_TEXTURE_CUBE_MAP_FACE_EXT: Self = Self(0x8cd3);
    pub const GL_FRAMEBUFFER_ATTACHMENT_TEXTURE_3D_ZOFFSET_EXT: Self = Self(0x8cd4);
    pub const GL_FRAMEBUFFER_ATTACHMENT_GREEN_SIZE: Self = Self(0x8213);
    pub const GL_FRAMEBUFFER_ATTACHMENT_OBJECT_NAME_OES: Self = Self(0x8cd1);
    pub const GL_FRAMEBUFFER_ATTACHMENT_COMPONENT_TYPE_EXT: Self = Self(0x8211);
    pub const GL_FRAMEBUFFER_ATTACHMENT_TEXTURE_CUBE_MAP_FACE_OES: Self = Self(0x8cd3);
    pub const GL_FRAMEBUFFER_ATTACHMENT_TEXTURE_NUM_VIEWS_OVR: Self = Self(0x9630);
    pub const GL_FRAMEBUFFER_ATTACHMENT_TEXTURE_LEVEL_EXT: Self = Self(0x8cd2);
    pub const GL_FRAMEBUFFER_ATTACHMENT_OBJECT_TYPE_EXT: Self = Self(0x8cd0);
    pub const GL_FRAMEBUFFER_ATTACHMENT_TEXTURE_LAYER_EXT: Self = Self(0x8cd4);
    pub const GL_FRAMEBUFFER_ATTACHMENT_OBJECT_TYPE_OES: Self = Self(0x8cd0);
    pub const GL_FRAMEBUFFER_ATTACHMENT_COMPONENT_TYPE: Self = Self(0x8211);
    pub const GL_FRAMEBUFFER_ATTACHMENT_LAYERED_EXT: Self = Self(0x8da7);
    pub const GL_FRAMEBUFFER_ATTACHMENT_OBJECT_NAME_EXT: Self = Self(0x8cd1);
    pub const GL_FRAMEBUFFER_ATTACHMENT_LAYERED_OES: Self = Self(0x8da7);
    pub const GL_FRAMEBUFFER_ATTACHMENT_BLUE_SIZE: Self = Self(0x8214);
    pub const GL_FRAMEBUFFER_ATTACHMENT_TEXTURE_SCALE_IMG: Self = Self(0x913f);
    pub const GL_FRAMEBUFFER_ATTACHMENT_OBJECT_NAME: Self = Self(0x8cd1);
    pub const GL_FRAMEBUFFER_ATTACHMENT_ALPHA_SIZE: Self = Self(0x8215);
    pub const GL_FRAMEBUFFER_ATTACHMENT_TEXTURE_LEVEL: Self = Self(0x8cd2);
    pub const GL_FRAMEBUFFER_ATTACHMENT_COLOR_ENCODING: Self = Self(0x8210);
    pub const GL_FRAMEBUFFER_ATTACHMENT_DEPTH_SIZE: Self = Self(0x8216);
    pub const GL_FRAMEBUFFER_ATTACHMENT_STENCIL_SIZE: Self = Self(0x8217);
    pub const GL_FRAMEBUFFER_ATTACHMENT_OBJECT_TYPE: Self = Self(0x8cd0);
    pub const GL_FRAMEBUFFER_ATTACHMENT_LAYERED: Self = Self(0x8da7);
    pub const GL_FRAMEBUFFER_ATTACHMENT_TEXTURE_BASE_VIEW_INDEX_OVR: Self = Self(0x9632);
    pub const GL_FRAMEBUFFER_ATTACHMENT_COLOR_ENCODING_EXT: Self = Self(0x8210);
    pub const GL_FRAMEBUFFER_ATTACHMENT_TEXTURE_LEVEL_OES: Self = Self(0x8cd2);
    pub const GL_FRAMEBUFFER_ATTACHMENT_TEXTURE_CUBE_MAP_FACE: Self = Self(0x8cd3);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CombinerComponentUsageNV(pub(crate) u64);
impl CombinerComponentUsageNV {
    pub const GL_ALPHA: Self = Self(0x1906);
    pub const GL_BLUE: Self = Self(0x1905);
    pub const GL_RGB: Self = Self(0x1907);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GetMinmaxParameterPNameEXT(pub(crate) u64);
impl GetMinmaxParameterPNameEXT {
    pub const GL_MINMAX_SINK: Self = Self(0x8030);
    pub const GL_MINMAX_FORMAT_EXT: Self = Self(0x802f);
    pub const GL_MINMAX_FORMAT: Self = Self(0x802f);
    pub const GL_MINMAX_SINK_EXT: Self = Self(0x8030);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PixelStoreParameter(pub(crate) u64);
impl PixelStoreParameter {
    pub const GL_PIXEL_TILE_GRID_HEIGHT_SGIX: Self = Self(0x8143);
    pub const GL_UNPACK_SWAP_BYTES: Self = Self(0x0cf0);
    pub const GL_PACK_ROW_LENGTH: Self = Self(0x0d02);
    pub const GL_PIXEL_TILE_WIDTH_SGIX: Self = Self(0x8140);
    pub const GL_PACK_SUBSAMPLE_RATE_SGIX: Self = Self(0x85a0);
    pub const GL_UNPACK_ROW_LENGTH_EXT: Self = Self(0x0cf2);
    pub const GL_UNPACK_SKIP_ROWS: Self = Self(0x0cf3);
    pub const GL_UNPACK_SUBSAMPLE_RATE_SGIX: Self = Self(0x85a1);
    pub const GL_PIXEL_TILE_GRID_DEPTH_SGIX: Self = Self(0x8144);
    pub const GL_UNPACK_LSB_FIRST: Self = Self(0x0cf1);
    pub const GL_PACK_SKIP_IMAGES: Self = Self(0x806b);
    pub const GL_PACK_IMAGE_DEPTH_SGIS: Self = Self(0x8131);
    pub const GL_UNPACK_ALIGNMENT: Self = Self(0x0cf5);
    pub const GL_PACK_IMAGE_HEIGHT: Self = Self(0x806c);
    pub const GL_UNPACK_IMAGE_HEIGHT_EXT: Self = Self(0x806e);
    pub const GL_UNPACK_RESAMPLE_SGIX: Self = Self(0x842f);
    pub const GL_PACK_SKIP_PIXELS: Self = Self(0x0d04);
    pub const GL_PIXEL_TILE_GRID_WIDTH_SGIX: Self = Self(0x8142);
    pub const GL_UNPACK_SKIP_PIXELS: Self = Self(0x0cf4);
    pub const GL_PACK_SKIP_VOLUMES_SGIS: Self = Self(0x8130);
    pub const GL_UNPACK_IMAGE_DEPTH_SGIS: Self = Self(0x8133);
    pub const GL_PIXEL_TILE_CACHE_SIZE_SGIX: Self = Self(0x8145);
    pub const GL_PACK_RESAMPLE_OML: Self = Self(0x8984);
    pub const GL_UNPACK_SKIP_IMAGES: Self = Self(0x806d);
    pub const GL_PACK_SKIP_ROWS: Self = Self(0x0d03);
    pub const GL_PACK_IMAGE_HEIGHT_EXT: Self = Self(0x806c);
    pub const GL_UNPACK_IMAGE_HEIGHT: Self = Self(0x806e);
    pub const GL_PIXEL_TILE_HEIGHT_SGIX: Self = Self(0x8141);
    pub const GL_UNPACK_SKIP_VOLUMES_SGIS: Self = Self(0x8132);
    pub const GL_PACK_ALIGNMENT: Self = Self(0x0d05);
    pub const GL_UNPACK_SKIP_IMAGES_EXT: Self = Self(0x806d);
    pub const GL_UNPACK_RESAMPLE_OML: Self = Self(0x8985);
    pub const GL_PACK_RESAMPLE_SGIX: Self = Self(0x842e);
    pub const GL_PACK_SKIP_IMAGES_EXT: Self = Self(0x806b);
    pub const GL_PACK_LSB_FIRST: Self = Self(0x0d01);
    pub const GL_PACK_SWAP_BYTES: Self = Self(0x0d00);
    pub const GL_UNPACK_ROW_LENGTH: Self = Self(0x0cf2);
    pub const GL_UNPACK_SKIP_PIXELS_EXT: Self = Self(0x0cf4);
    pub const GL_UNPACK_SKIP_ROWS_EXT: Self = Self(0x0cf3);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DrawElementsType(pub(crate) u64);
impl DrawElementsType {
    pub const GL_UNSIGNED_BYTE: Self = Self(0x1401);
    pub const GL_UNSIGNED_SHORT: Self = Self(0x1403);
    pub const GL_UNSIGNED_INT: Self = Self(0x1405);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GetPName(pub(crate) u64);
impl GetPName {
    pub const GL_MAX_TIMELINE_SEMAPHORE_VALUE_DIFFERENCE_NV: Self = Self(0x95b6);
    pub const GL_ALIASED_LINE_WIDTH_RANGE: Self = Self(0x846e);
    pub const GL_UNPACK_IMAGE_HEIGHT_EXT: Self = Self(0x806e);
    pub const GL_NUM_PROGRAM_BINARY_FORMATS: Self = Self(0x87fe);
    pub const GL_VIEWPORT: Self = Self(0x0ba2);
    pub const GL_POST_CONVOLUTION_GREEN_BIAS_EXT: Self = Self(0x8021);
    pub const GL_FOG_COLOR: Self = Self(0x0b66);
    pub const GL_ALPHA_BITS: Self = Self(0x0d55);
    pub const GL_MAX_UNIFORM_BUFFER_BINDINGS: Self = Self(0x8a2f);
    pub const GL_MAX_FRAGMENT_UNIFORM_VECTORS: Self = Self(0x8dfd);
    pub const GL_MAP1_GRID_DOMAIN: Self = Self(0x0dd0);
    pub const GL_PRIMITIVE_RESTART_INDEX: Self = Self(0x8f9e);
    pub const GL_PACK_ROW_LENGTH: Self = Self(0x0d02);
    pub const GL_PACK_IMAGE_HEIGHT: Self = Self(0x806c);
    pub const GL_TEXTURE_BINDING_RECTANGLE_NV: Self = Self(0x84f6);
    pub const GL_PACK_SKIP_PIXELS: Self = Self(0x0d04);
    pub const GL_ALPHA_TEST_FUNC_QCOM: Self = Self(0x0bc1);
    pub const GL_BLUE_BIAS: Self = Self(0x0d1b);
    pub const GL_CLIP_PLANE5: Self = Self(0x3005);
    pub const GL_POST_CONVOLUTION_ALPHA_SCALE_EXT: Self = Self(0x801f);
    pub const GL_COLOR_ARRAY_TYPE: Self = Self(0x8082);
    pub const GL_VERTEX_PRECLIP_SGIX: Self = Self(0x83ee);
    pub const GL_UNIFORM_BUFFER_SIZE: Self = Self(0x8a2a);
    pub const GL_DRAW_FRAMEBUFFER_BINDING: Self = Self(0x8ca6);
    pub const GL_TRANSFORM_FEEDBACK_BUFFER_BINDING: Self = Self(0x8c8f);
    pub const GL_READ_FRAMEBUFFER_BINDING: Self = Self(0x8caa);
    pub const GL_MAP2_INDEX: Self = Self(0x0db1);
    pub const GL_MAX_GEOMETRY_UNIFORM_COMPONENTS: Self = Self(0x8ddf);
    pub const GL_SHADING_RATE_IMAGE_PALETTE_COUNT_NV: Self = Self(0x95b2);
    pub const GL_UNPACK_LSB_FIRST: Self = Self(0x0cf1);
    pub const GL_MAX_VARYING_FLOATS: Self = Self(0x8b4b);
    pub const GL_MAP1_TEXTURE_COORD_2: Self = Self(0x0d94);
    pub const GL_UNPACK_CMYK_HINT_EXT: Self = Self(0x800f);
    pub const GL_BLEND_SRC_RGB: Self = Self(0x80c9);
    pub const GL_COLOR_ARRAY_STRIDE: Self = Self(0x8083);
    pub const GL_MAX_COMPUTE_WORK_GROUP_INVOCATIONS: Self = Self(0x90eb);
    pub const GL_MAX_TESS_CONTROL_ATOMIC_COUNTERS: Self = Self(0x92d3);
    pub const GL_MAX_FOG_FUNC_POINTS_SGIS: Self = Self(0x812c);
    pub const GL_EDGE_FLAG: Self = Self(0x0b43);
    pub const GL_SAMPLE_MASK_INVERT_SGIS: Self = Self(0x80ab);
    pub const GL_FOG_OFFSET_SGIX: Self = Self(0x8198);
    pub const GL_FRAGMENT_COLOR_MATERIAL_PARAMETER_SGIX: Self = Self(0x8403);
    pub const GL_MAP2_COLOR_4: Self = Self(0x0db0);
    pub const GL_INDEX_SHIFT: Self = Self(0x0d12);
    pub const GL_LIGHT5: Self = Self(0x4005);
    pub const GL_PIXEL_TEX_GEN_MODE_SGIX: Self = Self(0x832b);
    pub const GL_POST_COLOR_MATRIX_COLOR_TABLE_SGI: Self = Self(0x80d2);
    pub const GL_ALPHA_TEST_FUNC: Self = Self(0x0bc1);
    pub const GL_TEXTURE_BINDING_2D_MULTISAMPLE: Self = Self(0x9104);
    pub const GL_PROJECTION_STACK_DEPTH: Self = Self(0x0ba4);
    pub const GL_MAP1_TEXTURE_COORD_3: Self = Self(0x0d95);
    pub const GL_CURRENT_RASTER_TEXTURE_COORDS: Self = Self(0x0b06);
    pub const GL_UNPACK_SKIP_IMAGES_EXT: Self = Self(0x806d);
    pub const GL_POST_COLOR_MATRIX_ALPHA_SCALE_SGI: Self = Self(0x80b7);
    pub const GL_SHARED_TEXTURE_PALETTE_EXT: Self = Self(0x81fb);
    pub const GL_MAX_TESS_EVALUATION_ATOMIC_COUNTERS: Self = Self(0x92d4);
    pub const GL_MAX_VERTEX_UNIFORM_VECTORS: Self = Self(0x8dfb);
    pub const GL_LIST_MODE: Self = Self(0x0b30);
    pub const GL_PACK_SKIP_VOLUMES_SGIS: Self = Self(0x8130);
    pub const GL_MAX_CLIPMAP_DEPTH_SGIX: Self = Self(0x8177);
    pub const GL_ALPHA_SCALE: Self = Self(0x0d1c);
    pub const GL_LINE_WIDTH_RANGE: Self = Self(0x0b22);
    pub const GL_UNPACK_SKIP_VOLUMES_SGIS: Self = Self(0x8132);
    pub const GL_READ_BUFFER: Self = Self(0x0c02);
    pub const GL_AUTO_NORMAL: Self = Self(0x0d80);
    pub const GL_COLOR_MATERIAL: Self = Self(0x0b57);
    pub const GL_COLOR_WRITEMASK: Self = Self(0x0c23);
    pub const GL_MAX_VERTEX_UNIFORM_COMPONENTS: Self = Self(0x8b4a);
    pub const GL_MAX_COMPUTE_ATOMIC_COUNTER_BUFFERS: Self = Self(0x8264);
    pub const GL_MAX_TEXTURE_STACK_DEPTH: Self = Self(0x0d39);
    pub const GL_PROVOKING_VERTEX: Self = Self(0x8e4f);
    pub const GL_DEPTH_CLEAR_VALUE: Self = Self(0x0b73);
    pub const GL_MAX_COMBINED_GEOMETRY_UNIFORM_COMPONENTS: Self = Self(0x8a32);
    pub const GL_POST_COLOR_MATRIX_BLUE_BIAS_SGI: Self = Self(0x80ba);
    pub const GL_FOG_MODE: Self = Self(0x0b65);
    pub const GL_MAX_GEOMETRY_INPUT_COMPONENTS: Self = Self(0x9123);
    pub const GL_ALIASED_POINT_SIZE_RANGE: Self = Self(0x846d);
    pub const GL_MAX_COMPUTE_UNIFORM_BLOCKS: Self = Self(0x91bb);
    pub const GL_MIN_MAP_BUFFER_ALIGNMENT: Self = Self(0x90bc);
    pub const GL_NUM_DEVICE_UUIDS_EXT: Self = Self(0x9596);
    pub const GL_ZOOM_Y: Self = Self(0x0d17);
    pub const GL_COLOR_TABLE_SGI: Self = Self(0x80d0);
    pub const GL_STENCIL_REF: Self = Self(0x0b97);
    pub const GL_PACK_ALIGNMENT: Self = Self(0x0d05);
    pub const GL_SHADING_RATE_QCOM: Self = Self(0x96a4);
    pub const GL_FRONT_FACE: Self = Self(0x0b46);
    pub const GL_MAX_FRAGMENT_SHADER_STORAGE_BLOCKS: Self = Self(0x90da);
    pub const GL_PACK_IMAGE_HEIGHT_EXT: Self = Self(0x806c);
    pub const GL_LIGHT2: Self = Self(0x4002);
    pub const GL_COLOR_MATRIX_STACK_DEPTH_SGI: Self = Self(0x80b2);
    pub const GL_PIXEL_TILE_CACHE_SIZE_SGIX: Self = Self(0x8145);
    pub const GL_TEXTURE_BINDING_2D_ARRAY: Self = Self(0x8c1d);
    pub const GL_STENCIL_WRITEMASK: Self = Self(0x0b98);
    pub const GL_MAX_GEOMETRY_OUTPUT_COMPONENTS: Self = Self(0x9124);
    pub const GL_POLYGON_OFFSET_BIAS_EXT: Self = Self(0x8039);
    pub const GL_HISTOGRAM_EXT: Self = Self(0x8024);
    pub const GL_FRAMEZOOM_FACTOR_SGIX: Self = Self(0x818c);
    pub const GL_MAX_DUAL_SOURCE_DRAW_BUFFERS: Self = Self(0x88fc);
    pub const GL_IR_INSTRUMENT1_SGIX: Self = Self(0x817f);
    pub const GL_MAX_SHADER_STORAGE_BUFFER_BINDINGS: Self = Self(0x90dd);
    pub const GL_CURRENT_TEXTURE_COORDS: Self = Self(0x0b03);
    pub const GL_TEXTURE_COORD_ARRAY_COUNT_EXT: Self = Self(0x808b);
    pub const GL_MAX_COLOR_MATRIX_STACK_DEPTH_SGI: Self = Self(0x80b3);
    pub const GL_TEXTURE_BINDING_BUFFER: Self = Self(0x8c2c);
    pub const GL_MAX_VERTEX_ATTRIBS: Self = Self(0x8869);
    pub const GL_SHADER_STORAGE_BUFFER_SIZE: Self = Self(0x90d5);
    pub const GL_RED_BITS: Self = Self(0x0d52);
    pub const GL_VIEWPORT_BOUNDS_RANGE: Self = Self(0x825d);
    pub const GL_MAX_CLIPMAP_VIRTUAL_DEPTH_SGIX: Self = Self(0x8178);
    pub const GL_MAP2_GRID_SEGMENTS: Self = Self(0x0dd3);
    pub const GL_BLUE_SCALE: Self = Self(0x0d1a);
    pub const GL_VIEWPORT_SUBPIXEL_BITS: Self = Self(0x825c);
    pub const GL_ASYNC_HISTOGRAM_SGIX: Self = Self(0x832c);
    pub const GL_MAX_DEPTH_TEXTURE_SAMPLES: Self = Self(0x910f);
    pub const GL_FOG_END: Self = Self(0x0b64);
    pub const GL_MAP1_TEXTURE_COORD_4: Self = Self(0x0d96);
    pub const GL_INDEX_ARRAY_COUNT_EXT: Self = Self(0x8087);
    pub const GL_VERTEX_ARRAY_TYPE: Self = Self(0x807b);
    pub const GL_ALPHA_TEST_REF_QCOM: Self = Self(0x0bc2);
    pub const GL_LINE_STIPPLE_PATTERN: Self = Self(0x0b25);
    pub const GL_POINT_SIZE_GRANULARITY: Self = Self(0x0b13);
    pub const GL_VERTEX_PRECLIP_HINT_SGIX: Self = Self(0x83ef);
    pub const GL_PIXEL_TILE_GRID_HEIGHT_SGIX: Self = Self(0x8143);
    pub const GL_MAJOR_VERSION: Self = Self(0x821b);
    pub const GL_INDEX_CLEAR_VALUE: Self = Self(0x0c20);
    pub const GL_PIXEL_MAP_I_TO_I_SIZE: Self = Self(0x0cb0);
    pub const GL_ACCUM_CLEAR_VALUE: Self = Self(0x0b80);
    pub const GL_MAX_MODELVIEW_STACK_DEPTH: Self = Self(0x0d36);
    pub const GL_MAX_COMPUTE_TEXTURE_IMAGE_UNITS: Self = Self(0x91bc);
    pub const GL_DEFORMATIONS_MASK_SGIX: Self = Self(0x8196);
    pub const GL_POST_CONVOLUTION_COLOR_TABLE_SGI: Self = Self(0x80d1);
    pub const GL_POST_CONVOLUTION_RED_BIAS_EXT: Self = Self(0x8020);
    pub const GL_COLOR_MATRIX_SGI: Self = Self(0x80b1);
    pub const GL_MAX_ASYNC_READ_PIXELS_SGIX: Self = Self(0x8361);
    pub const GL_PIXEL_TILE_GRID_WIDTH_SGIX: Self = Self(0x8142);
    pub const GL_PIXEL_MAP_B_TO_B_SIZE: Self = Self(0x0cb8);
    pub const GL_FRAGMENT_LIGHTING_SGIX: Self = Self(0x8400);
    pub const GL_CULL_FACE: Self = Self(0x0b44);
    pub const GL_INDEX_WRITEMASK: Self = Self(0x0c21);
    pub const GL_MAP_COLOR: Self = Self(0x0d10);
    pub const GL_CONVOLUTION_HINT_SGIX: Self = Self(0x8316);
    pub const GL_FRAGMENT_SHADER_DERIVATIVE_HINT: Self = Self(0x8b8b);
    pub const GL_MAX_ASYNC_DRAW_PIXELS_SGIX: Self = Self(0x8360);
    pub const GL_LIGHT4: Self = Self(0x4004);
    pub const GL_FEEDBACK_BUFFER_TYPE: Self = Self(0x0df2);
    pub const GL_POST_TEXTURE_FILTER_BIAS_RANGE_SGIX: Self = Self(0x817b);
    pub const GL_MAP2_GRID_DOMAIN: Self = Self(0x0dd2);
    pub const GL_TEXTURE_BUFFER_OFFSET_ALIGNMENT: Self = Self(0x919f);
    pub const GL_STENCIL_TEST: Self = Self(0x0b90);
    pub const GL_CLIP_PLANE2: Self = Self(0x3002);
    pub const GL_POLYGON_MODE: Self = Self(0x0b40);
    pub const GL_COLOR_ARRAY_SIZE: Self = Self(0x8081);
    pub const GL_SAMPLE_COVERAGE_VALUE: Self = Self(0x80aa);
    pub const GL_VERTEX_ARRAY_BINDING: Self = Self(0x85b5);
    pub const GL_FOG_DENSITY: Self = Self(0x0b62);
    pub const GL_MODELVIEW0_MATRIX_EXT: Self = Self(0x0ba6);
    pub const GL_STENCIL_CLEAR_VALUE: Self = Self(0x0b91);
    pub const GL_TEXTURE_GEN_T: Self = Self(0x0c61);
    pub const GL_SHADING_RATE_IMAGE_PER_PRIMITIVE_NV: Self = Self(0x95b1);
    pub const GL_MAX_TESS_EVALUATION_UNIFORM_BLOCKS: Self = Self(0x8e8a);
    pub const GL_MAX_4D_TEXTURE_SIZE_SGIS: Self = Self(0x8138);
    pub const GL_TEXTURE_GEN_Q: Self = Self(0x0c63);
    pub const GL_VIEWPORT_INDEX_PROVOKING_VERTEX: Self = Self(0x825f);
    pub const GL_ASYNC_DRAW_PIXELS_SGIX: Self = Self(0x835d);
    pub const GL_MAP2_TEXTURE_COORD_3: Self = Self(0x0db5);
    pub const GL_PIXEL_MAP_G_TO_G_SIZE: Self = Self(0x0cb7);
    pub const GL_MAX_RENDERBUFFER_SIZE: Self = Self(0x84e8);
    pub const GL_SHADER_STORAGE_BUFFER_OFFSET_ALIGNMENT: Self = Self(0x90df);
    pub const GL_FOG_INDEX: Self = Self(0x0b61);
    pub const GL_POLYGON_OFFSET_FILL: Self = Self(0x8037);
    pub const GL_UNPACK_RESAMPLE_SGIX: Self = Self(0x842f);
    pub const GL_STENCIL_BACK_WRITEMASK: Self = Self(0x8ca5);
    pub const GL_MODELVIEW_MATRIX: Self = Self(0x0ba6);
    pub const GL_BLEND_COLOR_EXT: Self = Self(0x8005);
    pub const GL_ACCUM_RED_BITS: Self = Self(0x0d58);
    pub const GL_CLIP_PLANE0: Self = Self(0x3000);
    pub const GL_MAX_CUBE_MAP_TEXTURE_SIZE: Self = Self(0x851c);
    pub const GL_MAX_GEOMETRY_UNIFORM_BLOCKS: Self = Self(0x8a2c);
    pub const GL_MAX_ELEMENT_INDEX: Self = Self(0x8d6b);
    pub const GL_MAX_COMPUTE_SHADER_STORAGE_BLOCKS: Self = Self(0x90db);
    pub const GL_DRIVER_UUID_EXT: Self = Self(0x9598);
    pub const GL_MAX_INTEGER_SAMPLES: Self = Self(0x9110);
    pub const GL_REFERENCE_PLANE_SGIX: Self = Self(0x817d);
    pub const GL_MAX_ASYNC_HISTOGRAM_SGIX: Self = Self(0x832d);
    pub const GL_PIXEL_UNPACK_BUFFER_BINDING: Self = Self(0x88ef);
    pub const GL_MAX_PROGRAM_TEXEL_OFFSET: Self = Self(0x8905);
    pub const GL_MULTISAMPLE_SGIS: Self = Self(0x809d);
    pub const GL_FRAGMENT_LIGHT_MODEL_AMBIENT_SGIX: Self = Self(0x840a);
    pub const GL_TEXTURE_COMPRESSION_HINT: Self = Self(0x84ef);
    pub const GL_MAX_COMBINED_UNIFORM_BLOCKS: Self = Self(0x8a2e);
    pub const GL_DRAW_BUFFER_EXT: Self = Self(0x0c01);
    pub const GL_SCISSOR_TEST: Self = Self(0x0c11);
    pub const GL_INSTRUMENT_MEASUREMENTS_SGIX: Self = Self(0x8181);
    pub const GL_MAX_FRAGMENT_LIGHTS_SGIX: Self = Self(0x8404);
    pub const GL_MAP1_GRID_SEGMENTS: Self = Self(0x0dd1);
    pub const GL_POINT_SIZE_MAX_SGIS: Self = Self(0x8127);
    pub const GL_TEXTURE_2D: Self = Self(0x0de1);
    pub const GL_NUM_SHADER_BINARY_FORMATS: Self = Self(0x8df9);
    pub const GL_MAX_TESS_EVALUATION_SHADER_STORAGE_BLOCKS: Self = Self(0x90d9);
    pub const GL_PACK_IMAGE_DEPTH_SGIS: Self = Self(0x8131);
    pub const GL_MAX_GEOMETRY_TEXTURE_IMAGE_UNITS: Self = Self(0x8c29);
    pub const GL_LINE_SMOOTH_HINT: Self = Self(0x0c52);
    pub const GL_CONTEXT_FLAGS: Self = Self(0x821e);
    pub const GL_BLEND_EQUATION_RGB: Self = Self(0x8009);
    pub const GL_LINE_STIPPLE: Self = Self(0x0b24);
    pub const GL_CURRENT_RASTER_POSITION_VALID: Self = Self(0x0b08);
    pub const GL_BLEND_COLOR: Self = Self(0x8005);
    pub const GL_NORMAL_ARRAY: Self = Self(0x8075);
    pub const GL_PIXEL_MAP_A_TO_A_SIZE: Self = Self(0x0cb9);
    pub const GL_MAX_DEBUG_GROUP_STACK_DEPTH: Self = Self(0x826c);
    pub const GL_SAMPLE_PATTERN_SGIS: Self = Self(0x80ac);
    pub const GL_TEXTURE_STACK_DEPTH: Self = Self(0x0ba5);
    pub const GL_COLOR_ARRAY_COUNT_EXT: Self = Self(0x8084);
    pub const GL_POINT_SMOOTH: Self = Self(0x0b10);
    pub const GL_DEPTH_BIAS: Self = Self(0x0d1f);
    pub const GL_MAX_LABEL_LENGTH: Self = Self(0x82e8);
    pub const GL_SHADER_BINARY_FORMATS: Self = Self(0x8df8);
    pub const GL_PIXEL_MAP_I_TO_R_SIZE: Self = Self(0x0cb2);
    pub const GL_MAX_ATTRIB_STACK_DEPTH: Self = Self(0x0d35);
    pub const GL_PACK_RESAMPLE_SGIX: Self = Self(0x842e);
    pub const GL_TEXTURE_MATRIX: Self = Self(0x0ba8);
    pub const GL_TRANSFORM_FEEDBACK_BUFFER_START: Self = Self(0x8c84);
    pub const GL_MODELVIEW_STACK_DEPTH: Self = Self(0x0ba3);
    pub const GL_MAX_NAME_STACK_DEPTH: Self = Self(0x0d37);
    pub const GL_NORMAL_ARRAY_STRIDE: Self = Self(0x807f);
    pub const GL_TEXTURE_BINDING_1D_ARRAY: Self = Self(0x8c1c);
    pub const GL_FOG: Self = Self(0x0b60);
    pub const GL_MAP2_TEXTURE_COORD_1: Self = Self(0x0db3);
    pub const GL_MAX_UNIFORM_BLOCK_SIZE: Self = Self(0x8a30);
    pub const GL_IMPLEMENTATION_COLOR_READ_FORMAT: Self = Self(0x8b9b);
    pub const GL_LINE_SMOOTH: Self = Self(0x0b20);
    pub const GL_SPRITE_MODE_SGIX: Self = Self(0x8149);
    pub const GL_ALPHA_TEST_QCOM: Self = Self(0x0bc0);
    pub const GL_MAP1_INDEX: Self = Self(0x0d91);
    pub const GL_POINT_FADE_THRESHOLD_SIZE_SGIS: Self = Self(0x8128);
    pub const GL_MAX_FRAGMENT_UNIFORM_COMPONENTS: Self = Self(0x8b49);
    pub const GL_PROJECTION_MATRIX: Self = Self(0x0ba7);
    pub const GL_TEXTURE_BINDING_2D_MULTISAMPLE_ARRAY: Self = Self(0x9105);
    pub const GL_SMOOTH_POINT_SIZE_RANGE: Self = Self(0x0b12);
    pub const GL_ACCUM_BLUE_BITS: Self = Self(0x0d5a);
    pub const GL_INDEX_ARRAY_TYPE: Self = Self(0x8085);
    pub const GL_MAP2_NORMAL: Self = Self(0x0db2);
    pub const GL_MAX_FRAMEBUFFER_WIDTH: Self = Self(0x9315);
    pub const GL_CURRENT_NORMAL: Self = Self(0x0b02);
    pub const GL_CALLIGRAPHIC_FRAGMENT_SGIX: Self = Self(0x8183);
    pub const GL_UNPACK_SKIP_PIXELS: Self = Self(0x0cf4);
    pub const GL_MAX_TEXTURE_SIZE: Self = Self(0x0d33);
    pub const GL_MAP2_VERTEX_3: Self = Self(0x0db7);
    pub const GL_SPRITE_TRANSLATION_SGIX: Self = Self(0x814b);
    pub const GL_RED_BIAS: Self = Self(0x0d15);
    pub const GL_POST_CONVOLUTION_GREEN_SCALE_EXT: Self = Self(0x801d);
    pub const GL_UNPACK_SKIP_ROWS: Self = Self(0x0cf3);
    pub const GL_DEPTH_BITS: Self = Self(0x0d56);
    pub const GL_PIXEL_MAP_I_TO_G_SIZE: Self = Self(0x0cb3);
    pub const GL_CURRENT_RASTER_COLOR: Self = Self(0x0b04);
    pub const GL_MAX_COLOR_ATTACHMENTS: Self = Self(0x8cdf);
    pub const GL_MAP1_NORMAL: Self = Self(0x0d92);
    pub const GL_LINE_WIDTH: Self = Self(0x0b21);
    pub const GL_MAX_FRAGMENT_INPUT_COMPONENTS: Self = Self(0x9125);
    pub const GL_LINE_WIDTH_GRANULARITY: Self = Self(0x0b23);
    pub const GL_LIGHTING: Self = Self(0x0b50);
    pub const GL_STENCIL_PASS_DEPTH_PASS: Self = Self(0x0b96);
    pub const GL_LIGHT_MODEL_LOCAL_VIEWER: Self = Self(0x0b51);
    pub const GL_PIXEL_MAP_S_TO_S_SIZE: Self = Self(0x0cb1);
    pub const GL_UNPACK_ROW_LENGTH: Self = Self(0x0cf2);
    pub const GL_MAX_COMPUTE_UNIFORM_COMPONENTS: Self = Self(0x8263);
    pub const GL_POST_COLOR_MATRIX_ALPHA_BIAS_SGI: Self = Self(0x80bb);
    pub const GL_VERTEX_ARRAY_SIZE: Self = Self(0x807a);
    pub const GL_POST_CONVOLUTION_BLUE_BIAS_EXT: Self = Self(0x8022);
    pub const GL_PIXEL_PACK_BUFFER_BINDING: Self = Self(0x88ed);
    pub const GL_MAX_VARYING_COMPONENTS: Self = Self(0x8b4b);
    pub const GL_ALPHA_BIAS: Self = Self(0x0d1d);
    pub const GL_MAX_FRAGMENT_UNIFORM_BLOCKS: Self = Self(0x8a2d);
    pub const GL_FRAMEZOOM_SGIX: Self = Self(0x818b);
    pub const GL_MAX_COLOR_ATTACHMENTS_NV: Self = Self(0x8cdf);
    pub const GL_CURRENT_RASTER_INDEX: Self = Self(0x0b05);
    pub const GL_POLYGON_OFFSET_POINT: Self = Self(0x2a01);
    pub const GL_VERTEX_BINDING_OFFSET: Self = Self(0x82d7);
    pub const GL_LIGHT_MODEL_AMBIENT: Self = Self(0x0b53);
    pub const GL_UNPACK_IMAGE_HEIGHT: Self = Self(0x806e);
    pub const GL_POST_COLOR_MATRIX_GREEN_BIAS_SGI: Self = Self(0x80b9);
    pub const GL_DEBUG_GROUP_STACK_DEPTH: Self = Self(0x826d);
    pub const GL_MAX_VARYING_VECTORS: Self = Self(0x8dfc);
    pub const GL_BLUE_BITS: Self = Self(0x0d54);
    pub const GL_PACK_LSB_FIRST: Self = Self(0x0d01);
    pub const GL_TEXTURE_BINDING_RECTANGLE: Self = Self(0x84f6);
    pub const GL_ACTIVE_TEXTURE: Self = Self(0x84e0);
    pub const GL_UNPACK_SWAP_BYTES: Self = Self(0x0cf0);
    pub const GL_MAX_VERTEX_ATTRIB_BINDINGS: Self = Self(0x82da);
    pub const GL_EDGE_FLAG_ARRAY_COUNT_EXT: Self = Self(0x808d);
    pub const GL_ARRAY_BUFFER_BINDING: Self = Self(0x8894);
    pub const GL_VERTEX_BINDING_DIVISOR: Self = Self(0x82d6);
    pub const GL_LOGIC_OP_MODE: Self = Self(0x0bf0);
    pub const GL_PACK_CMYK_HINT_EXT: Self = Self(0x800e);
    pub const GL_NUM_COMPRESSED_TEXTURE_FORMATS: Self = Self(0x86a2);
    pub const GL_MAX_ACTIVE_LIGHTS_SGIX: Self = Self(0x8405);
    pub const GL_FOG_START: Self = Self(0x0b63);
    pub const GL_TEXTURE_BINDING_CUBE_MAP_OES: Self = Self(0x8514);
    pub const GL_SAMPLER_BINDING: Self = Self(0x8919);
    pub const GL_IMPLEMENTATION_COLOR_READ_TYPE: Self = Self(0x8b9a);
    pub const GL_COLOR_LOGIC_OP: Self = Self(0x0bf2);
    pub const GL_CURRENT_RASTER_DISTANCE: Self = Self(0x0b09);
    pub const GL_MAX_ARRAY_TEXTURE_LAYERS: Self = Self(0x88ff);
    pub const GL_INDEX_MODE: Self = Self(0x0c30);
    pub const GL_INDEX_BITS: Self = Self(0x0d51);
    pub const GL_LIST_INDEX: Self = Self(0x0b33);
    pub const GL_SAMPLE_BUFFERS_SGIS: Self = Self(0x80a8);
    pub const GL_MAX_PIXEL_MAP_TABLE: Self = Self(0x0d34);
    pub const GL_MAX_CLIENT_ATTRIB_STACK_DEPTH: Self = Self(0x0d3b);
    pub const GL_STENCIL_BACK_REF: Self = Self(0x8ca3);
    pub const GL_GREEN_BIAS: Self = Self(0x0d19);
    pub const GL_POINT_SIZE_RANGE: Self = Self(0x0b12);
    pub const GL_MINMAX_EXT: Self = Self(0x802e);
    pub const GL_PACK_SKIP_IMAGES: Self = Self(0x806b);
    pub const GL_SPRITE_AXIS_SGIX: Self = Self(0x814a);
    pub const GL_STENCIL_BACK_PASS_DEPTH_PASS: Self = Self(0x8803);
    pub const GL_PERSPECTIVE_CORRECTION_HINT: Self = Self(0x0c50);
    pub const GL_GENERATE_MIPMAP_HINT_SGIS: Self = Self(0x8192);
    pub const GL_STENCIL_BACK_PASS_DEPTH_FAIL: Self = Self(0x8802);
    pub const GL_ALPHA_TEST_REF: Self = Self(0x0bc2);
    pub const GL_SHADER_COMPILER: Self = Self(0x8dfa);
    pub const GL_TEXTURE_COORD_ARRAY: Self = Self(0x8078);
    pub const GL_PACK_SKIP_IMAGES_EXT: Self = Self(0x806b);
    pub const GL_SMOOTH_LINE_WIDTH_GRANULARITY: Self = Self(0x0b23);
    pub const GL_TEXTURE_COORD_ARRAY_TYPE: Self = Self(0x8089);
    pub const GL_MAX_TEXTURE_LOD_BIAS: Self = Self(0x84fd);
    pub const GL_MAX_CLIP_DISTANCES: Self = Self(0x0d32);
    pub const GL_MAX_VIEWPORTS: Self = Self(0x825b);
    pub const GL_RESCALE_NORMAL_EXT: Self = Self(0x803a);
    pub const GL_LOGIC_OP: Self = Self(0x0bf1);
    pub const GL_GREEN_BITS: Self = Self(0x0d53);
    pub const GL_SCISSOR_BOX: Self = Self(0x0c10);
    pub const GL_MAX_3D_TEXTURE_SIZE: Self = Self(0x8073);
    pub const GL_PIXEL_TILE_HEIGHT_SGIX: Self = Self(0x8141);
    pub const GL_RENDER_MODE: Self = Self(0x0c40);
    pub const GL_MAP1_TEXTURE_COORD_1: Self = Self(0x0d93);
    pub const GL_ASYNC_MARKER_SGIX: Self = Self(0x8329);
    pub const GL_MAX_TEXTURE_IMAGE_UNITS: Self = Self(0x8872);
    pub const GL_MOTION_ESTIMATION_SEARCH_BLOCK_Y_QCOM: Self = Self(0x8c91);
    pub const GL_PROGRAM_BINARY_FORMATS: Self = Self(0x87ff);
    pub const GL_CLIP_PLANE1: Self = Self(0x3001);
    pub const GL_PIXEL_TEXTURE_SGIS: Self = Self(0x8353);
    pub const GL_FRAGMENT_COLOR_MATERIAL_SGIX: Self = Self(0x8401);
    pub const GL_ZOOM_X: Self = Self(0x0d16);
    pub const GL_FRAGMENT_LIGHT_MODEL_TWO_SIDE_SGIX: Self = Self(0x8409);
    pub const GL_POLYGON_STIPPLE: Self = Self(0x0b42);
    pub const GL_TIMESTAMP: Self = Self(0x8e28);
    pub const GL_MAP2_TEXTURE_COORD_2: Self = Self(0x0db4);
    pub const GL_TEXTURE_BINDING_CUBE_MAP: Self = Self(0x8514);
    pub const GL_FRAGMENT_SHADER_FRAMEBUFFER_FETCH_MRT_ARM: Self = Self(0x8f66);
    pub const GL_TEXTURE_4D_SGIS: Self = Self(0x8134);
    pub const GL_TEXTURE_COORD_ARRAY_SIZE: Self = Self(0x8088);
    pub const GL_PIXEL_TILE_WIDTH_SGIX: Self = Self(0x8140);
    pub const GL_STENCIL_BACK_VALUE_MASK: Self = Self(0x8ca4);
    pub const GL_BLEND_EQUATION_EXT: Self = Self(0x8009);
    pub const GL_STENCIL_PASS_DEPTH_FAIL: Self = Self(0x0b95);
    pub const GL_GREEN_SCALE: Self = Self(0x0d18);
    pub const GL_DEVICE_UUID_EXT: Self = Self(0x9597);
    pub const GL_MAX_ELEMENTS_VERTICES: Self = Self(0x80e8);
    pub const GL_BLEND_DST_ALPHA: Self = Self(0x80ca);
    pub const GL_MAP_STENCIL: Self = Self(0x0d11);
    pub const GL_SAMPLES_SGIS: Self = Self(0x80a9);
    pub const GL_MAP2_VERTEX_4: Self = Self(0x0db8);
    pub const GL_MAX_TEXTURE_BUFFER_SIZE: Self = Self(0x8c2b);
    pub const GL_FRAGMENT_COLOR_MATERIAL_FACE_SGIX: Self = Self(0x8402);
    pub const GL_ATTRIB_STACK_DEPTH: Self = Self(0x0bb0);
    pub const GL_NAME_STACK_DEPTH: Self = Self(0x0d70);
    pub const GL_PIXEL_MAP_I_TO_B_SIZE: Self = Self(0x0cb4);
    pub const GL_UNPACK_ALIGNMENT: Self = Self(0x0cf5);
    pub const GL_MAX_DRAW_BUFFERS: Self = Self(0x8824);
    pub const GL_DEPTH_RANGE: Self = Self(0x0b70);
    pub const GL_POINT_SIZE_MIN_SGIS: Self = Self(0x8126);
    pub const GL_TEXTURE_BINDING_CUBE_MAP_ARB: Self = Self(0x8514);
    pub const GL_BLEND_EQUATION_ALPHA: Self = Self(0x883d);
    pub const GL_MAX_VERTEX_TEXTURE_IMAGE_UNITS: Self = Self(0x8b4c);
    pub const GL_SMOOTH_POINT_SIZE_GRANULARITY: Self = Self(0x0b13);
    pub const GL_RED_SCALE: Self = Self(0x0d14);
    pub const GL_VERTEX_ARRAY: Self = Self(0x8074);
    pub const GL_EDGE_FLAG_ARRAY_STRIDE: Self = Self(0x808c);
    pub const GL_FRAGMENT_LIGHT_MODEL_LOCAL_VIEWER_SGIX: Self = Self(0x8408);
    pub const GL_PROGRAM_POINT_SIZE: Self = Self(0x8642);
    pub const GL_CURRENT_INDEX: Self = Self(0x0b01);
    pub const GL_ELEMENT_ARRAY_BUFFER_BINDING: Self = Self(0x8895);
    pub const GL_ACCUM_ALPHA_BITS: Self = Self(0x0d5b);
    pub const GL_BLEND_DST: Self = Self(0x0be0);
    pub const GL_MAX_GEOMETRY_SHADER_STORAGE_BLOCKS: Self = Self(0x90d7);
    pub const GL_POINT_FADE_THRESHOLD_SIZE: Self = Self(0x8128);
    pub const GL_TEXTURE_BINDING_1D: Self = Self(0x8068);
    pub const GL_NORMALIZE: Self = Self(0x0ba1);
    pub const GL_MAX_VIEWPORT_DIMS: Self = Self(0x0d3a);
    pub const GL_AUX_BUFFERS: Self = Self(0x0c00);
    pub const GL_MAX_LIST_NESTING: Self = Self(0x0b31);
    pub const GL_DEPTH_SCALE: Self = Self(0x0d1e);
    pub const GL_MAX_VERTEX_UNIFORM_BLOCKS: Self = Self(0x8a2b);
    pub const GL_STENCIL_FAIL: Self = Self(0x0b94);
    pub const GL_STEREO: Self = Self(0x0c33);
    pub const GL_CLIENT_ATTRIB_STACK_DEPTH: Self = Self(0x0bb1);
    pub const GL_STENCIL_FUNC: Self = Self(0x0b92);
    pub const GL_VERTEX_ARRAY_STRIDE: Self = Self(0x807c);
    pub const GL_POST_COLOR_MATRIX_GREEN_SCALE_SGI: Self = Self(0x80b5);
    pub const GL_LINE_STIPPLE_REPEAT: Self = Self(0x0b26);
    pub const GL_LIGHT3: Self = Self(0x4003);
    pub const GL_MINOR_VERSION: Self = Self(0x821c);
    pub const GL_DEPTH_FUNC: Self = Self(0x0b74);
    pub const GL_LAYER_PROVOKING_VERTEX: Self = Self(0x825e);
    pub const GL_INDEX_ARRAY_STRIDE: Self = Self(0x8086);
    pub const GL_POLYGON_OFFSET_FACTOR: Self = Self(0x8038);
    pub const GL_ACCUM_GREEN_BITS: Self = Self(0x0d59);
    pub const GL_POLYGON_OFFSET_LINE: Self = Self(0x2a02);
    pub const GL_LIGHT0: Self = Self(0x4000);
    pub const GL_UNIFORM_BUFFER_START: Self = Self(0x8a29);
    pub const GL_MAX_COMBINED_SHADER_STORAGE_BLOCKS: Self = Self(0x90dc);
    pub const GL_MAX_COMBINED_ATOMIC_COUNTERS: Self = Self(0x92d7);
    pub const GL_SAMPLE_BUFFERS: Self = Self(0x80a8);
    pub const GL_LIGHT_MODEL_TWO_SIDE: Self = Self(0x0b52);
    pub const GL_FOG_HINT: Self = Self(0x0c54);
    pub const GL_VERTEX_BINDING_STRIDE: Self = Self(0x82d8);
    pub const GL_MATRIX_MODE: Self = Self(0x0ba0);
    pub const GL_SELECTION_BUFFER_SIZE: Self = Self(0x0df4);
    pub const GL_PIXEL_TILE_CACHE_INCREMENT_SGIX: Self = Self(0x813f);
    pub const GL_SUBPIXEL_BITS: Self = Self(0x0d50);
    pub const GL_MAX_ELEMENTS_INDICES: Self = Self(0x80e9);
    pub const GL_MOTION_ESTIMATION_SEARCH_BLOCK_X_QCOM: Self = Self(0x8c90);
    pub const GL_MAP1_VERTEX_3: Self = Self(0x0d97);
    pub const GL_POST_CONVOLUTION_ALPHA_BIAS_EXT: Self = Self(0x8023);
    pub const GL_CLIP_PLANE3: Self = Self(0x3003);
    pub const GL_TEXTURE_BINDING_2D: Self = Self(0x8069);
    pub const GL_POST_CONVOLUTION_RED_SCALE_EXT: Self = Self(0x801c);
    pub const GL_BLEND_DST_RGB: Self = Self(0x80c8);
    pub const GL_REFERENCE_PLANE_EQUATION_SGIX: Self = Self(0x817e);
    pub const GL_FETCH_PER_SAMPLE_ARM: Self = Self(0x8f65);
    pub const GL_MODELVIEW0_STACK_DEPTH_EXT: Self = Self(0x0ba3);
    pub const GL_PIXEL_MAP_I_TO_A_SIZE: Self = Self(0x0cb5);
    pub const GL_SPRITE_SGIX: Self = Self(0x8148);
    pub const GL_UNPACK_IMAGE_DEPTH_SGIS: Self = Self(0x8133);
    pub const GL_STENCIL_VALUE_MASK: Self = Self(0x0b93);
    pub const GL_CLIP_PLANE4: Self = Self(0x3004);
    pub const GL_LIGHT_ENV_MODE_SGIX: Self = Self(0x8407);
    pub const GL_CURRENT_PROGRAM: Self = Self(0x8b8d);
    pub const GL_TRANSFORM_FEEDBACK_BUFFER_SIZE: Self = Self(0x8c85);
    pub const GL_CONVOLUTION_2D_EXT: Self = Self(0x8011);
    pub const GL_PACK_SUBSAMPLE_RATE_SGIX: Self = Self(0x85a0);
    pub const GL_POLYGON_SMOOTH: Self = Self(0x0b41);
    pub const GL_CULL_FACE_MODE: Self = Self(0x0b45);
    pub const GL_MAX_LIGHTS: Self = Self(0x0d31);
    pub const GL_CONVOLUTION_1D_EXT: Self = Self(0x8010);
    pub const GL_ASYNC_READ_PIXELS_SGIX: Self = Self(0x835e);
    pub const GL_COLOR_CLEAR_VALUE: Self = Self(0x0c22);
    pub const GL_MAP2_TEXTURE_COORD_4: Self = Self(0x0db6);
    pub const GL_UNIFORM_BUFFER_BINDING: Self = Self(0x8a28);
    pub const GL_MAX_VERTEX_OUTPUT_COMPONENTS: Self = Self(0x9122);
    pub const GL_MAX_GEOMETRY_ATOMIC_COUNTERS: Self = Self(0x92d5);
    pub const GL_DEVICE_LUID_EXT: Self = Self(0x9599);
    pub const GL_UNPACK_SKIP_IMAGES: Self = Self(0x806d);
    pub const GL_POST_COLOR_MATRIX_RED_BIAS_SGI: Self = Self(0x80b8);
    pub const GL_TEXTURE_1D: Self = Self(0x0de0);
    pub const GL_SHADER_STORAGE_BUFFER_BINDING: Self = Self(0x90d3);
    pub const GL_DISTANCE_ATTENUATION_SGIS: Self = Self(0x8129);
    pub const GL_MAX_COMPUTE_WORK_GROUP_SIZE: Self = Self(0x91bf);
    pub const GL_SMOOTH_LINE_WIDTH_RANGE: Self = Self(0x0b22);
    pub const GL_SHADE_MODEL: Self = Self(0x0b54);
    pub const GL_MAX_COMPUTE_ATOMIC_COUNTERS: Self = Self(0x8265);
    pub const GL_DITHER: Self = Self(0x0bd0);
    pub const GL_MAX_TESS_CONTROL_UNIFORM_BLOCKS: Self = Self(0x8e89);
    pub const GL_NORMAL_ARRAY_TYPE: Self = Self(0x807e);
    pub const GL_DEPTH_TEST: Self = Self(0x0b71);
    pub const GL_RGBA_MODE: Self = Self(0x0c31);
    pub const GL_POST_TEXTURE_FILTER_SCALE_RANGE_SGIX: Self = Self(0x817c);
    pub const GL_MAX_COMBINED_VERTEX_UNIFORM_COMPONENTS: Self = Self(0x8a31);
    pub const GL_MAX_VERTEX_SHADER_STORAGE_BLOCKS: Self = Self(0x90d6);
    pub const GL_STENCIL_BACK_FAIL: Self = Self(0x8801);
    pub const GL_POLYGON_OFFSET_UNITS: Self = Self(0x2a00);
    pub const GL_MAX_COMBINED_FRAGMENT_UNIFORM_COMPONENTS: Self = Self(0x8a33);
    pub const GL_LIGHT1: Self = Self(0x4001);
    pub const GL_DEPTH_WRITEMASK: Self = Self(0x0b72);
    pub const GL_SAMPLE_ALPHA_TO_MASK_SGIS: Self = Self(0x809e);
    pub const GL_MAX_FRAMEBUFFER_HEIGHT: Self = Self(0x9316);
    pub const GL_CURRENT_RASTER_POSITION: Self = Self(0x0b07);
    pub const GL_MAX_PROJECTION_STACK_DEPTH: Self = Self(0x0d38);
    pub const GL_LIST_BASE: Self = Self(0x0b32);
    pub const GL_ASYNC_TEX_IMAGE_SGIX: Self = Self(0x835c);
    pub const GL_TEXTURE_BINDING_RECTANGLE_ARB: Self = Self(0x84f6);
    pub const GL_MAX_TESS_CONTROL_SHADER_STORAGE_BLOCKS: Self = Self(0x90d8);
    pub const GL_ALPHA_TEST: Self = Self(0x0bc0);
    pub const GL_TEXTURE_3D_BINDING_EXT: Self = Self(0x806a);
    pub const GL_MAX_COMBINED_COMPUTE_UNIFORM_COMPONENTS: Self = Self(0x8266);
    pub const GL_DETAIL_TEXTURE_2D_BINDING_SGIS: Self = Self(0x8096);
    pub const GL_BLEND_SRC_ALPHA: Self = Self(0x80cb);
    pub const GL_POST_COLOR_MATRIX_BLUE_SCALE_SGI: Self = Self(0x80b6);
    pub const GL_READ_BUFFER_NV: Self = Self(0x0c02);
    pub const GL_POLYGON_SMOOTH_HINT: Self = Self(0x0c53);
    pub const GL_POINT_SMOOTH_HINT: Self = Self(0x0c51);
    pub const GL_LIGHT6: Self = Self(0x4006);
    pub const GL_LIGHT_MODEL_COLOR_CONTROL: Self = Self(0x81f8);
    pub const GL_TEXTURE_BINDING_CUBE_MAP_EXT: Self = Self(0x8514);
    pub const GL_SEPARABLE_2D_EXT: Self = Self(0x8012);
    pub const GL_SAMPLE_ALPHA_TO_ONE_SGIS: Self = Self(0x809f);
    pub const GL_PACK_SKIP_ROWS: Self = Self(0x0d03);
    pub const GL_PIXEL_TILE_GRID_DEPTH_SGIX: Self = Self(0x8144);
    pub const GL_MAX_SAMPLE_MASK_WORDS: Self = Self(0x8e59);
    pub const GL_TEXTURE_GEN_R: Self = Self(0x0c62);
    pub const GL_POINT_SIZE: Self = Self(0x0b11);
    pub const GL_MAX_COLOR_TEXTURE_SAMPLES: Self = Self(0x910e);
    pub const GL_BLEND: Self = Self(0x0be2);
    pub const GL_MAX_FRAMEBUFFER_LAYERS: Self = Self(0x9317);
    pub const GL_FOG_FUNC_POINTS_SGIS: Self = Self(0x812b);
    pub const GL_CONTEXT_PROFILE_MASK: Self = Self(0x9126);
    pub const GL_MAX_VERTEX_ATOMIC_COUNTERS: Self = Self(0x92d2);
    pub const GL_INDEX_LOGIC_OP: Self = Self(0x0bf1);
    pub const GL_SAMPLES: Self = Self(0x80a9);
    pub const GL_MAX_CLIP_PLANES: Self = Self(0x0d32);
    pub const GL_MAX_FRAMEZOOM_FACTOR_SGIX: Self = Self(0x818d);
    pub const GL_PIXEL_TILE_BEST_ALIGNMENT_SGIX: Self = Self(0x813e);
    pub const GL_POST_CONVOLUTION_BLUE_SCALE_EXT: Self = Self(0x801e);
    pub const GL_SAMPLE_MASK_VALUE_SGIS: Self = Self(0x80aa);
    pub const GL_MAX_UNIFORM_LOCATIONS: Self = Self(0x826e);
    pub const GL_MAX_SERVER_WAIT_TIMEOUT: Self = Self(0x9111);
    pub const GL_NORMAL_ARRAY_COUNT_EXT: Self = Self(0x8080);
    pub const GL_FRAGMENT_LIGHT0_SGIX: Self = Self(0x840c);
    pub const GL_UNIFORM_BUFFER_OFFSET_ALIGNMENT: Self = Self(0x8a34);
    pub const GL_MAX_ASYNC_TEX_IMAGE_SGIX: Self = Self(0x835f);
    pub const GL_MIN_PROGRAM_TEXEL_OFFSET: Self = Self(0x8904);
    pub const GL_BLEND_SRC: Self = Self(0x0be1);
    pub const GL_STENCIL_BITS: Self = Self(0x0d57);
    pub const GL_VERTEX_ARRAY_COUNT_EXT: Self = Self(0x807d);
    pub const GL_MAP1_COLOR_4: Self = Self(0x0d90);
    pub const GL_FEEDBACK_BUFFER_SIZE: Self = Self(0x0df1);
    pub const GL_TEXTURE_3D_EXT: Self = Self(0x806f);
    pub const GL_PIXEL_TEX_GEN_SGIX: Self = Self(0x8139);
    pub const GL_TEXTURE_4D_BINDING_SGIS: Self = Self(0x814f);
    pub const GL_DOUBLEBUFFER: Self = Self(0x0c32);
    pub const GL_READ_BUFFER_EXT: Self = Self(0x0c02);
    pub const GL_MAX_COMBINED_TEXTURE_IMAGE_UNITS: Self = Self(0x8b4d);
    pub const GL_COLOR_ARRAY: Self = Self(0x8076);
    pub const GL_TEXTURE_COLOR_TABLE_SGI: Self = Self(0x80bc);
    pub const GL_COLOR_MATERIAL_PARAMETER: Self = Self(0x0b56);
    pub const GL_EDGE_FLAG_ARRAY: Self = Self(0x8079);
    pub const GL_FRAGMENT_LIGHT_MODEL_NORMAL_INTERPOLATION_SGIX: Self = Self(0x840b);
    pub const GL_DEVICE_NODE_MASK_EXT: Self = Self(0x959a);
    pub const GL_NUM_EXTENSIONS: Self = Self(0x821d);
    pub const GL_PIXEL_MAP_R_TO_R_SIZE: Self = Self(0x0cb6);
    pub const GL_PROGRAM_PIPELINE_BINDING: Self = Self(0x825a);
    pub const GL_PACK_SWAP_BYTES: Self = Self(0x0d00);
    pub const GL_SAMPLE_MASK_SGIS: Self = Self(0x80a0);
    pub const GL_COLOR_MATERIAL_FACE: Self = Self(0x0b55);
    pub const GL_INDEX_OFFSET: Self = Self(0x0d13);
    pub const GL_DRAW_BUFFER: Self = Self(0x0c01);
    pub const GL_INTERLACE_SGIX: Self = Self(0x8094);
    pub const GL_MAX_VERTEX_ATTRIB_RELATIVE_OFFSET: Self = Self(0x82d9);
    pub const GL_COMPRESSED_TEXTURE_FORMATS: Self = Self(0x86a3);
    pub const GL_MAX_RECTANGLE_TEXTURE_SIZE: Self = Self(0x84f8);
    pub const GL_RENDERBUFFER_BINDING: Self = Self(0x8ca7);
    pub const GL_MAX_FRAGMENT_ATOMIC_COUNTERS: Self = Self(0x92d6);
    pub const GL_MAX_COLOR_ATTACHMENTS_EXT: Self = Self(0x8cdf);
    pub const GL_UNPACK_SUBSAMPLE_RATE_SGIX: Self = Self(0x85a1);
    pub const GL_INDEX_ARRAY: Self = Self(0x8077);
    pub const GL_MAP1_VERTEX_4: Self = Self(0x0d98);
    pub const GL_LIGHT7: Self = Self(0x4007);
    pub const GL_SAMPLE_COVERAGE_INVERT: Self = Self(0x80ab);
    pub const GL_SHADER_STORAGE_BUFFER_START: Self = Self(0x90d4);
    pub const GL_DISPATCH_INDIRECT_BUFFER_BINDING: Self = Self(0x90ef);
    pub const GL_MAX_FRAMEBUFFER_SAMPLES: Self = Self(0x9318);
    pub const GL_FOG_OFFSET_VALUE_SGIX: Self = Self(0x8199);
    pub const GL_STENCIL_BACK_FUNC: Self = Self(0x8800);
    pub const GL_TEXTURE_BINDING_3D: Self = Self(0x806a);
    pub const GL_MAX_EVAL_ORDER: Self = Self(0x0d30);
    pub const GL_TEXTURE_GEN_S: Self = Self(0x0c60);
    pub const GL_CURRENT_COLOR: Self = Self(0x0b00);
    pub const GL_MAX_COMPUTE_WORK_GROUP_COUNT: Self = Self(0x91be);
    pub const GL_MAX_3D_TEXTURE_SIZE_EXT: Self = Self(0x8073);
    pub const GL_TEXTURE_COORD_ARRAY_STRIDE: Self = Self(0x808a);
    pub const GL_POST_COLOR_MATRIX_RED_SCALE_SGI: Self = Self(0x80b4);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TextureCoordName(pub(crate) u64);
impl TextureCoordName {
    pub const GL_R: Self = Self(0x2002);
    pub const GL_Q: Self = Self(0x2003);
    pub const GL_T: Self = Self(0x2001);
    pub const GL_S: Self = Self(0x2000);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MinmaxTargetEXT(pub(crate) u64);
impl MinmaxTargetEXT {
    pub const GL_MINMAX_EXT: Self = Self(0x802e);
    pub const GL_MINMAX: Self = Self(0x802e);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ParameterRangeEXT(pub(crate) u64);
impl ParameterRangeEXT {
    pub const GL_NORMALIZED_RANGE_EXT: Self = Self(0x87e0);
    pub const GL_FULL_RANGE_EXT: Self = Self(0x87e1);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AttributeType(pub(crate) u64);
impl AttributeType {
    pub const GL_FLOAT_MAT3_ARB: Self = Self(0x8b5b);
    pub const GL_FLOAT_MAT2: Self = Self(0x8b5a);
    pub const GL_DOUBLE_VEC2: Self = Self(0x8ffc);
    pub const GL_DOUBLE_MAT4x3: Self = Self(0x8f4e);
    pub const GL_FLOAT_MAT2_ARB: Self = Self(0x8b5a);
    pub const GL_INT_IMAGE_2D: Self = Self(0x9058);
    pub const GL_SAMPLER_1D_SHADOW: Self = Self(0x8b61);
    pub const GL_UNSIGNED_INT_SAMPLER_2D_MULTISAMPLE: Self = Self(0x910a);
    pub const GL_UNSIGNED_INT_IMAGE_3D: Self = Self(0x9064);
    pub const GL_DOUBLE_MAT2x3: Self = Self(0x8f49);
    pub const GL_FLOAT_VEC2_ARB: Self = Self(0x8b50);
    pub const GL_INT_SAMPLER_2D_RECT: Self = Self(0x8dcd);
    pub const GL_INT_SAMPLER_CUBE_MAP_ARRAY: Self = Self(0x900e);
    pub const GL_FLOAT_VEC3_ARB: Self = Self(0x8b51);
    pub const GL_SAMPLER_1D_ARB: Self = Self(0x8b5d);
    pub const GL_SAMPLER_3D_ARB: Self = Self(0x8b5f);
    pub const GL_UNSIGNED_INT_SAMPLER_1D: Self = Self(0x8dd1);
    pub const GL_SAMPLER_2D_SHADOW_ARB: Self = Self(0x8b62);
    pub const GL_INT_IMAGE_2D_RECT: Self = Self(0x905a);
    pub const GL_FLOAT_MAT3x4_NV: Self = Self(0x8b68);
    pub const GL_INT64_ARB: Self = Self(0x140e);
    pub const GL_FLOAT_MAT3x4: Self = Self(0x8b68);
    pub const GL_DOUBLE: Self = Self(0x140a);
    pub const GL_FLOAT_VEC3: Self = Self(0x8b51);
    pub const GL_INT_VEC2: Self = Self(0x8b53);
    pub const GL_DOUBLE_VEC3: Self = Self(0x8ffd);
    pub const GL_UNSIGNED_INT_IMAGE_2D_MULTISAMPLE: Self = Self(0x906b);
    pub const GL_IMAGE_BUFFER: Self = Self(0x9051);
    pub const GL_SAMPLER_CUBE_MAP_ARRAY_SHADOW: Self = Self(0x900d);
    pub const GL_UNSIGNED_INT_SAMPLER_2D_MULTISAMPLE_ARRAY: Self = Self(0x910d);
    pub const GL_UNSIGNED_INT64_VEC2_ARB: Self = Self(0x8ff5);
    pub const GL_INT_VEC4_ARB: Self = Self(0x8b55);
    pub const GL_INT_SAMPLER_3D: Self = Self(0x8dcb);
    pub const GL_INT_IMAGE_2D_MULTISAMPLE_ARRAY: Self = Self(0x9061);
    pub const GL_FLOAT_VEC4_ARB: Self = Self(0x8b52);
    pub const GL_INT_SAMPLER_1D_ARRAY: Self = Self(0x8dce);
    pub const GL_INT_SAMPLER_2D_ARRAY: Self = Self(0x8dcf);
    pub const GL_IMAGE_3D: Self = Self(0x904e);
    pub const GL_IMAGE_2D_ARRAY: Self = Self(0x9053);
    pub const GL_INT_IMAGE_1D: Self = Self(0x9057);
    pub const GL_INT_VEC3_ARB: Self = Self(0x8b54);
    pub const GL_SAMPLER_2D: Self = Self(0x8b5e);
    pub const GL_UNSIGNED_INT_IMAGE_2D: Self = Self(0x9063);
    pub const GL_IMAGE_CUBE: Self = Self(0x9050);
    pub const GL_SAMPLER_1D_SHADOW_ARB: Self = Self(0x8b61);
    pub const GL_UNSIGNED_INT_VEC2: Self = Self(0x8dc6);
    pub const GL_INT_IMAGE_CUBE: Self = Self(0x905b);
    pub const GL_DOUBLE_MAT3: Self = Self(0x8f47);
    pub const GL_SAMPLER_3D_OES: Self = Self(0x8b5f);
    pub const GL_FLOAT_MAT3x2_NV: Self = Self(0x8b67);
    pub const GL_UNSIGNED_INT_SAMPLER_CUBE: Self = Self(0x8dd4);
    pub const GL_INT_SAMPLER_BUFFER: Self = Self(0x8dd0);
    pub const GL_SAMPLER_CUBE_MAP_ARRAY: Self = Self(0x900c);
    pub const GL_IMAGE_2D_RECT: Self = Self(0x904f);
    pub const GL_INT_SAMPLER_2D_MULTISAMPLE: Self = Self(0x9109);
    pub const GL_SAMPLER_2D_RECT_SHADOW_ARB: Self = Self(0x8b64);
    pub const GL_FLOAT_MAT4x3: Self = Self(0x8b6a);
    pub const GL_IMAGE_1D: Self = Self(0x904c);
    pub const GL_DOUBLE_MAT3x2: Self = Self(0x8f4b);
    pub const GL_UNSIGNED_INT64_VEC3_ARB: Self = Self(0x8ff6);
    pub const GL_BOOL_VEC4: Self = Self(0x8b59);
    pub const GL_UNSIGNED_INT_SAMPLER_2D_RECT: Self = Self(0x8dd5);
    pub const GL_SAMPLER_3D: Self = Self(0x8b5f);
    pub const GL_INT: Self = Self(0x1404);
    pub const GL_INT_SAMPLER_CUBE: Self = Self(0x8dcc);
    pub const GL_UNSIGNED_INT_IMAGE_CUBE_MAP_ARRAY: Self = Self(0x906a);
    pub const GL_UNSIGNED_INT: Self = Self(0x1405);
    pub const GL_FLOAT_MAT2x4: Self = Self(0x8b66);
    pub const GL_DOUBLE_MAT4x2: Self = Self(0x8f4d);
    pub const GL_UNSIGNED_INT64_NV: Self = Self(0x140f);
    pub const GL_FLOAT_MAT4_ARB: Self = Self(0x8b5c);
    pub const GL_SAMPLER_BUFFER: Self = Self(0x8dc2);
    pub const GL_UNSIGNED_INT_IMAGE_2D_RECT: Self = Self(0x9065);
    pub const GL_DOUBLE_MAT3x4: Self = Self(0x8f4c);
    pub const GL_FLOAT_MAT2x3_NV: Self = Self(0x8b65);
    pub const GL_DOUBLE_VEC4: Self = Self(0x8ffe);
    pub const GL_BOOL_ARB: Self = Self(0x8b56);
    pub const GL_UNSIGNED_INT_SAMPLER_2D_ARRAY: Self = Self(0x8dd7);
    pub const GL_IMAGE_2D: Self = Self(0x904d);
    pub const GL_SAMPLER_2D_RECT_ARB: Self = Self(0x8b63);
    pub const GL_FLOAT_MAT4x2: Self = Self(0x8b69);
    pub const GL_FLOAT_MAT4x2_NV: Self = Self(0x8b69);
    pub const GL_UNSIGNED_INT_IMAGE_2D_MULTISAMPLE_ARRAY: Self = Self(0x906c);
    pub const GL_INT_SAMPLER_2D: Self = Self(0x8dca);
    pub const GL_FLOAT_VEC4: Self = Self(0x8b52);
    pub const GL_BOOL_VEC2_ARB: Self = Self(0x8b57);
    pub const GL_UNSIGNED_INT_SAMPLER_3D: Self = Self(0x8dd3);
    pub const GL_DOUBLE_MAT2: Self = Self(0x8f46);
    pub const GL_SAMPLER_2D_SHADOW_EXT: Self = Self(0x8b62);
    pub const GL_INT_IMAGE_BUFFER: Self = Self(0x905c);
    pub const GL_IMAGE_CUBE_MAP_ARRAY: Self = Self(0x9054);
    pub const GL_INT64_NV: Self = Self(0x140e);
    pub const GL_SAMPLER_1D: Self = Self(0x8b5d);
    pub const GL_IMAGE_2D_MULTISAMPLE_ARRAY: Self = Self(0x9056);
    pub const GL_INT_IMAGE_2D_ARRAY: Self = Self(0x905e);
    pub const GL_DOUBLE_MAT2x4: Self = Self(0x8f4a);
    pub const GL_UNSIGNED_INT_VEC3: Self = Self(0x8dc7);
    pub const GL_SAMPLER_2D_MULTISAMPLE: Self = Self(0x9108);
    pub const GL_FLOAT_MAT2x4_NV: Self = Self(0x8b66);
    pub const GL_UNSIGNED_INT_IMAGE_1D: Self = Self(0x9062);
    pub const GL_BOOL_VEC3: Self = Self(0x8b58);
    pub const GL_SAMPLER_2D_ARB: Self = Self(0x8b5e);
    pub const GL_UNSIGNED_INT_SAMPLER_BUFFER: Self = Self(0x8dd8);
    pub const GL_SAMPLER_CUBE: Self = Self(0x8b60);
    pub const GL_SAMPLER_2D_ARRAY_SHADOW: Self = Self(0x8dc4);
    pub const GL_FLOAT_MAT4: Self = Self(0x8b5c);
    pub const GL_INT_VEC4: Self = Self(0x8b55);
    pub const GL_UNSIGNED_INT_SAMPLER_2D: Self = Self(0x8dd2);
    pub const GL_SAMPLER_2D_MULTISAMPLE_ARRAY: Self = Self(0x910b);
    pub const GL_SAMPLER_2D_SHADOW: Self = Self(0x8b62);
    pub const GL_INT_IMAGE_1D_ARRAY: Self = Self(0x905d);
    pub const GL_FLOAT_MAT2x3: Self = Self(0x8b65);
    pub const GL_UNSIGNED_INT_IMAGE_2D_ARRAY: Self = Self(0x9069);
    pub const GL_FLOAT_MAT3x2: Self = Self(0x8b67);
    pub const GL_BOOL_VEC3_ARB: Self = Self(0x8b58);
    pub const GL_INT64_VEC2_ARB: Self = Self(0x8fe9);
    pub const GL_UNSIGNED_INT_SAMPLER_1D_ARRAY: Self = Self(0x8dd6);
    pub const GL_INT_IMAGE_3D: Self = Self(0x9059);
    pub const GL_INT_SAMPLER_1D: Self = Self(0x8dc9);
    pub const GL_SAMPLER_CUBE_ARB: Self = Self(0x8b60);
    pub const GL_SAMPLER_2D_RECT: Self = Self(0x8b63);
    pub const GL_INT_VEC2_ARB: Self = Self(0x8b53);
    pub const GL_INT_VEC3: Self = Self(0x8b54);
    pub const GL_SAMPLER_1D_ARRAY_SHADOW: Self = Self(0x8dc3);
    pub const GL_UNSIGNED_INT_VEC4: Self = Self(0x8dc8);
    pub const GL_INT_IMAGE_CUBE_MAP_ARRAY: Self = Self(0x905f);
    pub const GL_UNSIGNED_INT_IMAGE_CUBE: Self = Self(0x9066);
    pub const GL_BOOL: Self = Self(0x8b56);
    pub const GL_UNSIGNED_INT64_ARB: Self = Self(0x140f);
    pub const GL_BOOL_VEC4_ARB: Self = Self(0x8b59);
    pub const GL_UNSIGNED_INT_SAMPLER_CUBE_MAP_ARRAY: Self = Self(0x900f);
    pub const GL_DOUBLE_MAT4: Self = Self(0x8f48);
    pub const GL_INT64_VEC3_ARB: Self = Self(0x8fea);
    pub const GL_INT64_VEC4_ARB: Self = Self(0x8feb);
    pub const GL_IMAGE_1D_ARRAY: Self = Self(0x9052);
    pub const GL_IMAGE_2D_MULTISAMPLE: Self = Self(0x9055);
    pub const GL_FLOAT: Self = Self(0x1406);
    pub const GL_UNSIGNED_INT64_VEC4_ARB: Self = Self(0x8ff7);
    pub const GL_FLOAT_MAT4x3_NV: Self = Self(0x8b6a);
    pub const GL_INT_SAMPLER_2D_MULTISAMPLE_ARRAY: Self = Self(0x910c);
    pub const GL_FLOAT_MAT3: Self = Self(0x8b5b);
    pub const GL_BOOL_VEC2: Self = Self(0x8b57);
    pub const GL_SAMPLER_2D_RECT_SHADOW: Self = Self(0x8b64);
    pub const GL_UNSIGNED_INT_IMAGE_BUFFER: Self = Self(0x9067);
    pub const GL_INT_IMAGE_2D_MULTISAMPLE: Self = Self(0x9060);
    pub const GL_FLOAT_VEC2: Self = Self(0x8b50);
    pub const GL_UNSIGNED_INT_IMAGE_1D_ARRAY: Self = Self(0x9068);
    pub const GL_SAMPLER_CUBE_SHADOW: Self = Self(0x8dc5);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CombinerRegisterNV(pub(crate) u64);
impl CombinerRegisterNV {
    pub const GL_DISCARD_NV: Self = Self(0x8530);
    pub const GL_SPARE0_NV: Self = Self(0x852e);
    pub const GL_SPARE1_NV: Self = Self(0x852f);
    pub const GL_TEXTURE1_ARB: Self = Self(0x84c1);
    pub const GL_TEXTURE0_ARB: Self = Self(0x84c0);
    pub const GL_PRIMARY_COLOR_NV: Self = Self(0x852c);
    pub const GL_SECONDARY_COLOR_NV: Self = Self(0x852d);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PathElementType(pub(crate) u64);
impl PathElementType {
    pub const GL_UTF8_NV: Self = Self(0x909a);
    pub const GL_UTF16_NV: Self = Self(0x909b);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SpriteParameterNameSGIX(pub(crate) u64);
impl SpriteParameterNameSGIX {
    pub const GL_SPRITE_MODE_SGIX: Self = Self(0x8149);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CombinerScaleNV(pub(crate) u64);
impl CombinerScaleNV {
    pub const GL_NONE: Self = Self(0);
    pub const GL_SCALE_BY_ONE_HALF_NV: Self = Self(0x8540);
    pub const GL_SCALE_BY_FOUR_NV: Self = Self(0x853f);
    pub const GL_SCALE_BY_TWO_NV: Self = Self(0x853e);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ObjectTypeAPPLE(pub(crate) u64);
impl ObjectTypeAPPLE {
    pub const GL_DRAW_PIXELS_APPLE: Self = Self(0x8a0a);
    pub const GL_FENCE_APPLE: Self = Self(0x8a0b);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PathListMode(pub(crate) u64);
impl PathListMode {
    pub const GL_FIRST_TO_REST_NV: Self = Self(0x90af);
    pub const GL_ADJACENT_PAIRS_NV: Self = Self(0x90ae);
    pub const GL_ACCUM_ADJACENT_PAIRS_NV: Self = Self(0x90ad);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct IndexMaterialParameterEXT(pub(crate) u64);
impl IndexMaterialParameterEXT {
    pub const GL_INDEX_OFFSET: Self = Self(0x0d13);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GetMapQuery(pub(crate) u64);
impl GetMapQuery {
    pub const GL_COEFF: Self = Self(0x0a00);
    pub const GL_DOMAIN: Self = Self(0x0a02);
    pub const GL_ORDER: Self = Self(0x0a01);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CombinerVariableNV(pub(crate) u64);
impl CombinerVariableNV {
    pub const GL_VARIABLE_D_NV: Self = Self(0x8526);
    pub const GL_VARIABLE_E_NV: Self = Self(0x8527);
    pub const GL_VARIABLE_C_NV: Self = Self(0x8525);
    pub const GL_VARIABLE_F_NV: Self = Self(0x8528);
    pub const GL_VARIABLE_G_NV: Self = Self(0x8529);
    pub const GL_VARIABLE_A_NV: Self = Self(0x8523);
    pub const GL_VARIABLE_B_NV: Self = Self(0x8524);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PixelTransferParameter(pub(crate) u64);
impl PixelTransferParameter {
    pub const GL_GREEN_BIAS: Self = Self(0x0d19);
    pub const GL_BLUE_SCALE: Self = Self(0x0d1a);
    pub const GL_INDEX_OFFSET: Self = Self(0x0d13);
    pub const GL_RED_BIAS: Self = Self(0x0d15);
    pub const GL_POST_COLOR_MATRIX_GREEN_SCALE_SGI: Self = Self(0x80b5);
    pub const GL_POST_COLOR_MATRIX_BLUE_SCALE: Self = Self(0x80b6);
    pub const GL_POST_CONVOLUTION_GREEN_BIAS: Self = Self(0x8021);
    pub const GL_POST_COLOR_MATRIX_BLUE_SCALE_SGI: Self = Self(0x80b6);
    pub const GL_POST_COLOR_MATRIX_ALPHA_SCALE: Self = Self(0x80b7);
    pub const GL_POST_CONVOLUTION_ALPHA_BIAS: Self = Self(0x8023);
    pub const GL_POST_COLOR_MATRIX_RED_BIAS: Self = Self(0x80b8);
    pub const GL_POST_COLOR_MATRIX_RED_BIAS_SGI: Self = Self(0x80b8);
    pub const GL_POST_COLOR_MATRIX_ALPHA_BIAS: Self = Self(0x80bb);
    pub const GL_ALPHA_SCALE: Self = Self(0x0d1c);
    pub const GL_POST_CONVOLUTION_BLUE_SCALE: Self = Self(0x801e);
    pub const GL_POST_CONVOLUTION_BLUE_BIAS: Self = Self(0x8022);
    pub const GL_POST_COLOR_MATRIX_BLUE_BIAS_SGI: Self = Self(0x80ba);
    pub const GL_POST_CONVOLUTION_GREEN_SCALE_EXT: Self = Self(0x801d);
    pub const GL_DEPTH_BIAS: Self = Self(0x0d1f);
    pub const GL_POST_COLOR_MATRIX_ALPHA_BIAS_SGI: Self = Self(0x80bb);
    pub const GL_INDEX_SHIFT: Self = Self(0x0d12);
    pub const GL_BLUE_BIAS: Self = Self(0x0d1b);
    pub const GL_POST_COLOR_MATRIX_GREEN_BIAS: Self = Self(0x80b9);
    pub const GL_POST_CONVOLUTION_ALPHA_SCALE_EXT: Self = Self(0x801f);
    pub const GL_POST_CONVOLUTION_GREEN_SCALE: Self = Self(0x801d);
    pub const GL_POST_COLOR_MATRIX_BLUE_BIAS: Self = Self(0x80ba);
    pub const GL_ALPHA_BIAS: Self = Self(0x0d1d);
    pub const GL_POST_COLOR_MATRIX_RED_SCALE_SGI: Self = Self(0x80b4);
    pub const GL_RED_SCALE: Self = Self(0x0d14);
    pub const GL_POST_CONVOLUTION_BLUE_SCALE_EXT: Self = Self(0x801e);
    pub const GL_POST_CONVOLUTION_BLUE_BIAS_EXT: Self = Self(0x8022);
    pub const GL_POST_CONVOLUTION_RED_SCALE_EXT: Self = Self(0x801c);
    pub const GL_POST_CONVOLUTION_GREEN_BIAS_EXT: Self = Self(0x8021);
    pub const GL_POST_CONVOLUTION_ALPHA_BIAS_EXT: Self = Self(0x8023);
    pub const GL_POST_CONVOLUTION_ALPHA_SCALE: Self = Self(0x801f);
    pub const GL_GREEN_SCALE: Self = Self(0x0d18);
    pub const GL_POST_CONVOLUTION_RED_BIAS: Self = Self(0x8020);
    pub const GL_MAP_STENCIL: Self = Self(0x0d11);
    pub const GL_DEPTH_SCALE: Self = Self(0x0d1e);
    pub const GL_MAP_COLOR: Self = Self(0x0d10);
    pub const GL_POST_COLOR_MATRIX_RED_SCALE: Self = Self(0x80b4);
    pub const GL_POST_CONVOLUTION_RED_SCALE: Self = Self(0x801c);
    pub const GL_POST_COLOR_MATRIX_ALPHA_SCALE_SGI: Self = Self(0x80b7);
    pub const GL_POST_COLOR_MATRIX_GREEN_BIAS_SGI: Self = Self(0x80b9);
    pub const GL_POST_COLOR_MATRIX_GREEN_SCALE: Self = Self(0x80b5);
    pub const GL_POST_CONVOLUTION_RED_BIAS_EXT: Self = Self(0x8020);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StencilFaceDirection(pub(crate) u64);
impl StencilFaceDirection {
    pub const GL_FRONT_AND_BACK: Self = Self(0x0408);
    pub const GL_FRONT: Self = Self(0x0404);
    pub const GL_BACK: Self = Self(0x0405);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VertexAttribType(pub(crate) u64);
impl VertexAttribType {
    pub const GL_UNSIGNED_INT_10F_11F_11F_REV: Self = Self(0x8c3b);
    pub const GL_INT_2_10_10_10_REV: Self = Self(0x8d9f);
    pub const GL_UNSIGNED_SHORT: Self = Self(0x1403);
    pub const GL_UNSIGNED_INT_2_10_10_10_REV: Self = Self(0x8368);
    pub const GL_INT: Self = Self(0x1404);
    pub const GL_BYTE: Self = Self(0x1400);
    pub const GL_UNSIGNED_INT: Self = Self(0x1405);
    pub const GL_FLOAT: Self = Self(0x1406);
    pub const GL_DOUBLE: Self = Self(0x140a);
    pub const GL_UNSIGNED_BYTE: Self = Self(0x1401);
    pub const GL_HALF_FLOAT: Self = Self(0x140b);
    pub const GL_SHORT: Self = Self(0x1402);
    pub const GL_FIXED: Self = Self(0x140c);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PixelCopyType(pub(crate) u64);
impl PixelCopyType {
    pub const GL_COLOR_EXT: Self = Self(0x1800);
    pub const GL_STENCIL_EXT: Self = Self(0x1802);
    pub const GL_COLOR: Self = Self(0x1800);
    pub const GL_DEPTH: Self = Self(0x1801);
    pub const GL_DEPTH_EXT: Self = Self(0x1801);
    pub const GL_STENCIL: Self = Self(0x1802);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VertexAttribEnum(pub(crate) u64);
impl VertexAttribEnum {
    pub const GL_VERTEX_ATTRIB_ARRAY_ENABLED: Self = Self(0x8622);
    pub const GL_VERTEX_ATTRIB_ARRAY_BUFFER_BINDING: Self = Self(0x889f);
    pub const GL_VERTEX_ATTRIB_ARRAY_TYPE: Self = Self(0x8625);
    pub const GL_VERTEX_ATTRIB_ARRAY_INTEGER: Self = Self(0x88fd);
    pub const GL_VERTEX_ATTRIB_ARRAY_STRIDE: Self = Self(0x8624);
    pub const GL_VERTEX_ATTRIB_ARRAY_SIZE: Self = Self(0x8623);
    pub const GL_VERTEX_ATTRIB_ARRAY_DIVISOR: Self = Self(0x88fe);
    pub const GL_CURRENT_VERTEX_ATTRIB: Self = Self(0x8626);
    pub const GL_VERTEX_ATTRIB_ARRAY_NORMALIZED: Self = Self(0x886a);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PathParameter(pub(crate) u64);
impl PathParameter {
    pub const GL_PATH_COORD_COUNT_NV: Self = Self(0x909e);
    pub const GL_PATH_TERMINAL_END_CAP_NV: Self = Self(0x9078);
    pub const GL_PATH_CLIENT_LENGTH_NV: Self = Self(0x907f);
    pub const GL_PATH_MITER_LIMIT_NV: Self = Self(0x907a);
    pub const GL_PATH_FILL_MASK_NV: Self = Self(0x9081);
    pub const GL_PATH_INITIAL_END_CAP_NV: Self = Self(0x9077);
    pub const GL_PATH_OBJECT_BOUNDING_BOX_NV: Self = Self(0x908a);
    pub const GL_PATH_DASH_OFFSET_NV: Self = Self(0x907e);
    pub const GL_PATH_JOIN_STYLE_NV: Self = Self(0x9079);
    pub const GL_PATH_STROKE_COVER_MODE_NV: Self = Self(0x9083);
    pub const GL_PATH_STROKE_MASK_NV: Self = Self(0x9084);
    pub const GL_PATH_END_CAPS_NV: Self = Self(0x9076);
    pub const GL_PATH_DASH_CAPS_NV: Self = Self(0x907b);
    pub const GL_PATH_COMMAND_COUNT_NV: Self = Self(0x909d);
    pub const GL_PATH_DASH_ARRAY_COUNT_NV: Self = Self(0x909f);
    pub const GL_PATH_FILL_COVER_MODE_NV: Self = Self(0x9082);
    pub const GL_PATH_DASH_OFFSET_RESET_NV: Self = Self(0x90b4);
    pub const GL_PATH_COMPUTED_LENGTH_NV: Self = Self(0x90a0);
    pub const GL_PATH_STROKE_BOUNDING_BOX_NV: Self = Self(0x90a2);
    pub const GL_PATH_INITIAL_DASH_CAP_NV: Self = Self(0x907c);
    pub const GL_PATH_FILL_MODE_NV: Self = Self(0x9080);
    pub const GL_PATH_TERMINAL_DASH_CAP_NV: Self = Self(0x907d);
    pub const GL_PATH_FILL_BOUNDING_BOX_NV: Self = Self(0x90a1);
    pub const GL_PATH_STROKE_WIDTH_NV: Self = Self(0x9075);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct IndexFunctionEXT(pub(crate) u64);
impl IndexFunctionEXT {
    pub const GL_LESS: Self = Self(0x0201);
    pub const GL_GEQUAL: Self = Self(0x0206);
    pub const GL_GREATER: Self = Self(0x0204);
    pub const GL_NEVER: Self = Self(0x0200);
    pub const GL_NOTEQUAL: Self = Self(0x0205);
    pub const GL_ALWAYS: Self = Self(0x0207);
    pub const GL_LEQUAL: Self = Self(0x0203);
    pub const GL_EQUAL: Self = Self(0x0202);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ListMode(pub(crate) u64);
impl ListMode {
    pub const GL_COMPILE: Self = Self(0x1300);
    pub const GL_COMPILE_AND_EXECUTE: Self = Self(0x1301);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ReplacementCodeTypeSUN(pub(crate) u64);
impl ReplacementCodeTypeSUN {
    pub const GL_UNSIGNED_SHORT: Self = Self(0x1403);
    pub const GL_UNSIGNED_INT: Self = Self(0x1405);
    pub const GL_UNSIGNED_BYTE: Self = Self(0x1401);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ColorPointerType(pub(crate) u64);
impl ColorPointerType {
    pub const GL_UNSIGNED_BYTE: Self = Self(0x1401);
    pub const GL_UNSIGNED_INT: Self = Self(0x1405);
    pub const GL_BYTE: Self = Self(0x1400);
    pub const GL_UNSIGNED_SHORT: Self = Self(0x1403);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FramebufferTarget(pub(crate) u64);
impl FramebufferTarget {
    pub const GL_FRAMEBUFFER: Self = Self(0x8d40);
    pub const GL_FRAMEBUFFER_OES: Self = Self(0x8d40);
    pub const GL_DRAW_FRAMEBUFFER: Self = Self(0x8ca9);
    pub const GL_READ_FRAMEBUFFER: Self = Self(0x8ca8);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DebugSeverity(pub(crate) u64);
impl DebugSeverity {
    pub const GL_DEBUG_SEVERITY_LOW: Self = Self(0x9148);
    pub const GL_DEBUG_SEVERITY_HIGH: Self = Self(0x9146);
    pub const GL_DEBUG_SEVERITY_NOTIFICATION: Self = Self(0x826b);
    pub const GL_DONT_CARE: Self = Self(0x1100);
    pub const GL_DEBUG_SEVERITY_MEDIUM: Self = Self(0x9147);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ShaderBinaryFormat(pub(crate) u64);
impl ShaderBinaryFormat {
    pub const GL_SGX_BINARY_IMG: Self = Self(0x8c0a);
    pub const GL_SHADER_BINARY_VIV: Self = Self(0x8fc4);
    pub const GL_SHADER_BINARY_DMP: Self = Self(0x9250);
    pub const GL_SHADER_BINARY_FORMAT_SPIR_V: Self = Self(0x9551);
    pub const GL_MALI_SHADER_BINARY_ARM: Self = Self(0x8f60);
    pub const GL_GCCSO_SHADER_BINARY_FJ: Self = Self(0x9260);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LightEnvModeSGIX(pub(crate) u64);
impl LightEnvModeSGIX {
    pub const GL_ADD: Self = Self(0x0104);
    pub const GL_REPLACE: Self = Self(0x1e01);
    pub const GL_MODULATE: Self = Self(0x2100);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SamplerParameterF(pub(crate) u64);
impl SamplerParameterF {
    pub const GL_TEXTURE_UNNORMALIZED_COORDINATES_ARM: Self = Self(0x8f6a);
    pub const GL_TEXTURE_MIN_LOD: Self = Self(0x813a);
    pub const GL_TEXTURE_MAX_LOD: Self = Self(0x813b);
    pub const GL_TEXTURE_MAX_ANISOTROPY: Self = Self(0x84fe);
    pub const GL_TEXTURE_BORDER_COLOR: Self = Self(0x1004);
    pub const GL_TEXTURE_LOD_BIAS: Self = Self(0x8501);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MapQuery(pub(crate) u64);
impl MapQuery {
    pub const GL_DOMAIN: Self = Self(0x0a02);
    pub const GL_ORDER: Self = Self(0x0a01);
    pub const GL_COEFF: Self = Self(0x0a00);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FeedbackType(pub(crate) u64);
impl FeedbackType {
    pub const GL_2D: Self = Self(0x0600);
    pub const GL_3D_COLOR_TEXTURE: Self = Self(0x0603);
    pub const GL_4D_COLOR_TEXTURE: Self = Self(0x0604);
    pub const GL_3D: Self = Self(0x0601);
    pub const GL_3D_COLOR: Self = Self(0x0602);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LightModelColorControl(pub(crate) u64);
impl LightModelColorControl {
    pub const GL_SINGLE_COLOR: Self = Self(0x81f9);
    pub const GL_SEPARATE_SPECULAR_COLOR: Self = Self(0x81fa);
    pub const GL_SEPARATE_SPECULAR_COLOR_EXT: Self = Self(0x81fa);
    pub const GL_SINGLE_COLOR_EXT: Self = Self(0x81f9);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CombinerParameterNV(pub(crate) u64);
impl CombinerParameterNV {
    pub const GL_COMBINER_COMPONENT_USAGE_NV: Self = Self(0x8544);
    pub const GL_COMBINER_INPUT_NV: Self = Self(0x8542);
    pub const GL_COMBINER_MAPPING_NV: Self = Self(0x8543);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MeshMode2(pub(crate) u64);
impl MeshMode2 {
    pub const GL_FILL: Self = Self(0x1b02);
    pub const GL_LINE: Self = Self(0x1b01);
    pub const GL_POINT: Self = Self(0x1b00);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BufferAccessARB(pub(crate) u64);
impl BufferAccessARB {
    pub const GL_READ_ONLY: Self = Self(0x88b8);
    pub const GL_WRITE_ONLY: Self = Self(0x88b9);
    pub const GL_READ_WRITE: Self = Self(0x88ba);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MaterialParameter(pub(crate) u64);
impl MaterialParameter {
    pub const GL_EMISSION: Self = Self(0x1600);
    pub const GL_DIFFUSE: Self = Self(0x1201);
    pub const GL_SPECULAR: Self = Self(0x1202);
    pub const GL_AMBIENT: Self = Self(0x1200);
    pub const GL_SHININESS: Self = Self(0x1601);
    pub const GL_AMBIENT_AND_DIFFUSE: Self = Self(0x1602);
    pub const GL_COLOR_INDEXES: Self = Self(0x1603);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DepthStencilTextureMode(pub(crate) u64);
impl DepthStencilTextureMode {
    pub const GL_STENCIL_INDEX: Self = Self(0x1901);
    pub const GL_DEPTH_COMPONENT: Self = Self(0x1902);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ImageTransformTargetHP(pub(crate) u64);
impl ImageTransformTargetHP {
    pub const GL_IMAGE_TRANSFORM_2D_HP: Self = Self(0x8161);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AtomicCounterBufferPName(pub(crate) u64);
impl AtomicCounterBufferPName {
    pub const GL_ATOMIC_COUNTER_BUFFER_REFERENCED_BY_TESS_CONTROL_SHADER: Self = Self(0x92c8);
    pub const GL_ATOMIC_COUNTER_BUFFER_REFERENCED_BY_COMPUTE_SHADER: Self = Self(0x90ed);
    pub const GL_ATOMIC_COUNTER_BUFFER_ACTIVE_ATOMIC_COUNTERS: Self = Self(0x92c5);
    pub const GL_ATOMIC_COUNTER_BUFFER_REFERENCED_BY_FRAGMENT_SHADER: Self = Self(0x92cb);
    pub const GL_ATOMIC_COUNTER_BUFFER_ACTIVE_ATOMIC_COUNTER_INDICES: Self = Self(0x92c6);
    pub const GL_ATOMIC_COUNTER_BUFFER_BINDING: Self = Self(0x92c1);
    pub const GL_ATOMIC_COUNTER_BUFFER_DATA_SIZE: Self = Self(0x92c4);
    pub const GL_ATOMIC_COUNTER_BUFFER_REFERENCED_BY_VERTEX_SHADER: Self = Self(0x92c7);
    pub const GL_ATOMIC_COUNTER_BUFFER_REFERENCED_BY_TESS_EVALUATION_SHADER: Self = Self(0x92c9);
    pub const GL_ATOMIC_COUNTER_BUFFER_REFERENCED_BY_GEOMETRY_SHADER: Self = Self(0x92ca);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GetHistogramParameterPNameEXT(pub(crate) u64);
impl GetHistogramParameterPNameEXT {
    pub const GL_HISTOGRAM_FORMAT_EXT: Self = Self(0x8027);
    pub const GL_HISTOGRAM_RED_SIZE: Self = Self(0x8028);
    pub const GL_HISTOGRAM_RED_SIZE_EXT: Self = Self(0x8028);
    pub const GL_HISTOGRAM_LUMINANCE_SIZE_EXT: Self = Self(0x802c);
    pub const GL_HISTOGRAM_ALPHA_SIZE_EXT: Self = Self(0x802b);
    pub const GL_HISTOGRAM_SINK: Self = Self(0x802d);
    pub const GL_HISTOGRAM_WIDTH: Self = Self(0x8026);
    pub const GL_HISTOGRAM_LUMINANCE_SIZE: Self = Self(0x802c);
    pub const GL_HISTOGRAM_GREEN_SIZE_EXT: Self = Self(0x8029);
    pub const GL_HISTOGRAM_SINK_EXT: Self = Self(0x802d);
    pub const GL_HISTOGRAM_WIDTH_EXT: Self = Self(0x8026);
    pub const GL_HISTOGRAM_BLUE_SIZE: Self = Self(0x802a);
    pub const GL_HISTOGRAM_GREEN_SIZE: Self = Self(0x8029);
    pub const GL_HISTOGRAM_ALPHA_SIZE: Self = Self(0x802b);
    pub const GL_HISTOGRAM_FORMAT: Self = Self(0x8027);
    pub const GL_HISTOGRAM_BLUE_SIZE_EXT: Self = Self(0x802a);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SyncStatus(pub(crate) u64);
impl SyncStatus {
    pub const GL_TIMEOUT_EXPIRED: Self = Self(0x911b);
    pub const GL_CONDITION_SATISFIED: Self = Self(0x911c);
    pub const GL_WAIT_FAILED: Self = Self(0x911d);
    pub const GL_ALREADY_SIGNALED: Self = Self(0x911a);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MapTypeNV(pub(crate) u64);
impl MapTypeNV {
    pub const GL_DOUBLE: Self = Self(0x140a);
    pub const GL_FLOAT: Self = Self(0x1406);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct QueryParameterName(pub(crate) u64);
impl QueryParameterName {
    pub const GL_QUERY_COUNTER_BITS: Self = Self(0x8864);
    pub const GL_CURRENT_QUERY: Self = Self(0x8865);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FramebufferFetchNoncoherent(pub(crate) u64);
impl FramebufferFetchNoncoherent {
    pub const GL_FRAMEBUFFER_FETCH_NONCOHERENT_QCOM: Self = Self(0x96a2);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FramebufferParameterName(pub(crate) u64);
impl FramebufferParameterName {
    pub const GL_FRAMEBUFFER_DEFAULT_WIDTH: Self = Self(0x9310);
    pub const GL_FRAMEBUFFER_DEFAULT_FIXED_SAMPLE_LOCATIONS: Self = Self(0x9314);
    pub const GL_FRAMEBUFFER_DEFAULT_SAMPLES: Self = Self(0x9313);
    pub const GL_FRAMEBUFFER_DEFAULT_HEIGHT: Self = Self(0x9311);
    pub const GL_FRAMEBUFFER_DEFAULT_LAYERS: Self = Self(0x9312);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VertexShaderStorageTypeEXT(pub(crate) u64);
impl VertexShaderStorageTypeEXT {
    pub const GL_INVARIANT_EXT: Self = Self(0x87c2);
    pub const GL_LOCAL_EXT: Self = Self(0x87c4);
    pub const GL_VARIANT_EXT: Self = Self(0x87c1);
    pub const GL_LOCAL_CONSTANT_EXT: Self = Self(0x87c3);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SwizzleOpATI(pub(crate) u64);
impl SwizzleOpATI {
    pub const GL_SWIZZLE_STQ_DQ_ATI: Self = Self(0x8979);
    pub const GL_SWIZZLE_STQ_ATI: Self = Self(0x8977);
    pub const GL_SWIZZLE_STR_DR_ATI: Self = Self(0x8978);
    pub const GL_SWIZZLE_STR_ATI: Self = Self(0x8976);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CopyImageSubDataTarget(pub(crate) u64);
impl CopyImageSubDataTarget {
    pub const GL_TEXTURE_2D_MULTISAMPLE_ARRAY: Self = Self(0x9102);
    pub const GL_TEXTURE_1D_ARRAY: Self = Self(0x8c18);
    pub const GL_TEXTURE_2D: Self = Self(0x0de1);
    pub const GL_TEXTURE_CUBE_MAP_ARRAY: Self = Self(0x9009);
    pub const GL_TEXTURE_2D_MULTISAMPLE: Self = Self(0x9100);
    pub const GL_TEXTURE_CUBE_MAP: Self = Self(0x8513);
    pub const GL_RENDERBUFFER: Self = Self(0x8d41);
    pub const GL_TEXTURE_1D: Self = Self(0x0de0);
    pub const GL_TEXTURE_RECTANGLE: Self = Self(0x84f5);
    pub const GL_TEXTURE_3D: Self = Self(0x806f);
    pub const GL_TEXTURE_2D_ARRAY: Self = Self(0x8c1a);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct InternalFormatPName(pub(crate) u64);
impl InternalFormatPName {
    pub const GL_MAX_HEIGHT: Self = Self(0x827f);
    pub const GL_TEXTURE_SHADOW: Self = Self(0x82a1);
    pub const GL_SHADER_IMAGE_LOAD: Self = Self(0x82a4);
    pub const GL_SHADER_IMAGE_ATOMIC: Self = Self(0x82a6);
    pub const GL_IMAGE_TEXEL_SIZE: Self = Self(0x82a7);
    pub const GL_INTERNALFORMAT_DEPTH_TYPE: Self = Self(0x827c);
    pub const GL_NUM_SURFACE_COMPRESSION_FIXED_RATES_EXT: Self = Self(0x8f6e);
    pub const GL_IMAGE_COMPATIBILITY_CLASS: Self = Self(0x82a8);
    pub const GL_INTERNALFORMAT_GREEN_SIZE: Self = Self(0x8272);
    pub const GL_COMPUTE_TEXTURE: Self = Self(0x82a0);
    pub const GL_IMAGE_FORMAT_COMPATIBILITY_TYPE: Self = Self(0x90c7);
    pub const GL_INTERNALFORMAT_ALPHA_TYPE: Self = Self(0x827b);
    pub const GL_VERTEX_TEXTURE: Self = Self(0x829b);
    pub const GL_SRGB_WRITE: Self = Self(0x8298);
    pub const GL_COLOR_ENCODING: Self = Self(0x8296);
    pub const GL_MAX_DEPTH: Self = Self(0x8280);
    pub const GL_INTERNALFORMAT_DEPTH_SIZE: Self = Self(0x8275);
    pub const GL_INTERNALFORMAT_BLUE_TYPE: Self = Self(0x827a);
    pub const GL_COLOR_RENDERABLE: Self = Self(0x8286);
    pub const GL_TESS_CONTROL_TEXTURE: Self = Self(0x829c);
    pub const GL_AUTO_GENERATE_MIPMAP: Self = Self(0x8295);
    pub const GL_GENERATE_MIPMAP: Self = Self(0x8191);
    pub const GL_FRAMEBUFFER_RENDERABLE_LAYERED: Self = Self(0x828a);
    pub const GL_TEXTURE_COMPRESSED_BLOCK_SIZE: Self = Self(0x82b3);
    pub const GL_INTERNALFORMAT_GREEN_TYPE: Self = Self(0x8279);
    pub const GL_COLOR_COMPONENTS: Self = Self(0x8283);
    pub const GL_TEXTURE_IMAGE_TYPE: Self = Self(0x8290);
    pub const GL_INTERNALFORMAT_SUPPORTED: Self = Self(0x826f);
    pub const GL_READ_PIXELS_TYPE: Self = Self(0x828e);
    pub const GL_IMAGE_PIXEL_FORMAT: Self = Self(0x82a9);
    pub const GL_SIMULTANEOUS_TEXTURE_AND_STENCIL_TEST: Self = Self(0x82ad);
    pub const GL_SIMULTANEOUS_TEXTURE_AND_STENCIL_WRITE: Self = Self(0x82af);
    pub const GL_IMAGE_PIXEL_TYPE: Self = Self(0x82aa);
    pub const GL_SAMPLES: Self = Self(0x80a9);
    pub const GL_TEXTURE_COMPRESSED: Self = Self(0x86a1);
    pub const GL_INTERNALFORMAT_PREFERRED: Self = Self(0x8270);
    pub const GL_TEXTURE_IMAGE_FORMAT: Self = Self(0x828f);
    pub const GL_CLEAR_TEXTURE: Self = Self(0x9365);
    pub const GL_GET_TEXTURE_IMAGE_TYPE: Self = Self(0x8292);
    pub const GL_TEXTURE_GATHER: Self = Self(0x82a2);
    pub const GL_NUM_SAMPLE_COUNTS: Self = Self(0x9380);
    pub const GL_INTERNALFORMAT_BLUE_SIZE: Self = Self(0x8273);
    pub const GL_TEXTURE_GATHER_SHADOW: Self = Self(0x82a3);
    pub const GL_MAX_LAYERS: Self = Self(0x8281);
    pub const GL_READ_PIXELS: Self = Self(0x828c);
    pub const GL_INTERNALFORMAT_RED_SIZE: Self = Self(0x8271);
    pub const GL_SRGB_READ: Self = Self(0x8297);
    pub const GL_INTERNALFORMAT_STENCIL_TYPE: Self = Self(0x827d);
    pub const GL_DEPTH_RENDERABLE: Self = Self(0x8287);
    pub const GL_GEOMETRY_TEXTURE: Self = Self(0x829e);
    pub const GL_TEXTURE_COMPRESSED_BLOCK_HEIGHT: Self = Self(0x82b2);
    pub const GL_SIMULTANEOUS_TEXTURE_AND_DEPTH_TEST: Self = Self(0x82ac);
    pub const GL_FRAMEBUFFER_RENDERABLE: Self = Self(0x8289);
    pub const GL_TEXTURE_VIEW: Self = Self(0x82b5);
    pub const GL_READ_PIXELS_FORMAT: Self = Self(0x828d);
    pub const GL_TEXTURE_COMPRESSED_BLOCK_WIDTH: Self = Self(0x82b1);
    pub const GL_VIEW_COMPATIBILITY_CLASS: Self = Self(0x82b6);
    pub const GL_CLEAR_BUFFER: Self = Self(0x82b4);
    pub const GL_MAX_WIDTH: Self = Self(0x827e);
    pub const GL_FILTER: Self = Self(0x829a);
    pub const GL_SHADER_IMAGE_STORE: Self = Self(0x82a5);
    pub const GL_GET_TEXTURE_IMAGE_FORMAT: Self = Self(0x8291);
    pub const GL_INTERNALFORMAT_RED_TYPE: Self = Self(0x8278);
    pub const GL_MIPMAP: Self = Self(0x8293);
    pub const GL_STENCIL_RENDERABLE: Self = Self(0x8288);
    pub const GL_SIMULTANEOUS_TEXTURE_AND_DEPTH_WRITE: Self = Self(0x82ae);
    pub const GL_INTERNALFORMAT_ALPHA_SIZE: Self = Self(0x8274);
    pub const GL_INTERNALFORMAT_SHARED_SIZE: Self = Self(0x8277);
    pub const GL_FRAMEBUFFER_BLEND: Self = Self(0x828b);
    pub const GL_INTERNALFORMAT_STENCIL_SIZE: Self = Self(0x8276);
    pub const GL_TESS_EVALUATION_TEXTURE: Self = Self(0x829d);
    pub const GL_FRAGMENT_TEXTURE: Self = Self(0x829f);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LightName(pub(crate) u64);
impl LightName {
    pub const GL_LIGHT5: Self = Self(0x4005);
    pub const GL_FRAGMENT_LIGHT1_SGIX: Self = Self(0x840d);
    pub const GL_LIGHT4: Self = Self(0x4004);
    pub const GL_FRAGMENT_LIGHT6_SGIX: Self = Self(0x8412);
    pub const GL_FRAGMENT_LIGHT2_SGIX: Self = Self(0x840e);
    pub const GL_LIGHT1: Self = Self(0x4001);
    pub const GL_LIGHT7: Self = Self(0x4007);
    pub const GL_LIGHT2: Self = Self(0x4002);
    pub const GL_LIGHT0: Self = Self(0x4000);
    pub const GL_FRAGMENT_LIGHT0_SGIX: Self = Self(0x840c);
    pub const GL_FRAGMENT_LIGHT3_SGIX: Self = Self(0x840f);
    pub const GL_FRAGMENT_LIGHT7_SGIX: Self = Self(0x8413);
    pub const GL_LIGHT3: Self = Self(0x4003);
    pub const GL_FRAGMENT_LIGHT5_SGIX: Self = Self(0x8411);
    pub const GL_FRAGMENT_LIGHT4_SGIX: Self = Self(0x8410);
    pub const GL_LIGHT6: Self = Self(0x4006);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PathCoordType(pub(crate) u64);
impl PathCoordType {
    pub const GL_VERTICAL_LINE_TO_NV: Self = Self(0x08);
    pub const GL_CIRCULAR_TANGENT_ARC_TO_NV: Self = Self(0xfc);
    pub const GL_RESTART_PATH_NV: Self = Self(0xf0);
    pub const GL_LARGE_CCW_ARC_TO_NV: Self = Self(0x16);
    pub const GL_RELATIVE_CUBIC_CURVE_TO_NV: Self = Self(0x0d);
    pub const GL_RELATIVE_SMOOTH_QUADRATIC_CURVE_TO_NV: Self = Self(0x0f);
    pub const GL_RELATIVE_ROUNDED_RECT_NV: Self = Self(0xe9);
    pub const GL_RELATIVE_LINE_TO_NV: Self = Self(0x05);
    pub const GL_RELATIVE_ARC_TO_NV: Self = Self(0xff);
    pub const GL_RECT_NV: Self = Self(0xf6);
    pub const GL_DUP_FIRST_CUBIC_CURVE_TO_NV: Self = Self(0xf2);
    pub const GL_RELATIVE_MOVE_TO_NV: Self = Self(0x03);
    pub const GL_SMALL_CCW_ARC_TO_NV: Self = Self(0x12);
    pub const GL_RELATIVE_CONIC_CURVE_TO_NV: Self = Self(0x1b);
    pub const GL_CLOSE_PATH_NV: Self = Self(0x00);
    pub const GL_CUBIC_CURVE_TO_NV: Self = Self(0x0c);
    pub const GL_RELATIVE_ROUNDED_RECT2_NV: Self = Self(0xeb);
    pub const GL_SMOOTH_QUADRATIC_CURVE_TO_NV: Self = Self(0x0e);
    pub const GL_SMALL_CW_ARC_TO_NV: Self = Self(0x14);
    pub const GL_ROUNDED_RECT_NV: Self = Self(0xe8);
    pub const GL_RELATIVE_LARGE_CCW_ARC_TO_NV: Self = Self(0x17);
    pub const GL_RELATIVE_ROUNDED_RECT8_NV: Self = Self(0xef);
    pub const GL_ARC_TO_NV: Self = Self(0xfe);
    pub const GL_RELATIVE_HORIZONTAL_LINE_TO_NV: Self = Self(0x07);
    pub const GL_RELATIVE_RECT_NV: Self = Self(0xf7);
    pub const GL_RELATIVE_QUADRATIC_CURVE_TO_NV: Self = Self(0x0b);
    pub const GL_RELATIVE_LARGE_CW_ARC_TO_NV: Self = Self(0x19);
    pub const GL_QUADRATIC_CURVE_TO_NV: Self = Self(0x0a);
    pub const GL_CIRCULAR_CCW_ARC_TO_NV: Self = Self(0xf8);
    pub const GL_MOVE_TO_NV: Self = Self(0x02);
    pub const GL_SMOOTH_CUBIC_CURVE_TO_NV: Self = Self(0x10);
    pub const GL_LINE_TO_NV: Self = Self(0x04);
    pub const GL_ROUNDED_RECT2_NV: Self = Self(0xea);
    pub const GL_LARGE_CW_ARC_TO_NV: Self = Self(0x18);
    pub const GL_ROUNDED_RECT4_NV: Self = Self(0xec);
    pub const GL_RELATIVE_ROUNDED_RECT4_NV: Self = Self(0xed);
    pub const GL_HORIZONTAL_LINE_TO_NV: Self = Self(0x06);
    pub const GL_RELATIVE_SMALL_CCW_ARC_TO_NV: Self = Self(0x13);
    pub const GL_RELATIVE_SMALL_CW_ARC_TO_NV: Self = Self(0x15);
    pub const GL_ROUNDED_RECT8_NV: Self = Self(0xee);
    pub const GL_DUP_LAST_CUBIC_CURVE_TO_NV: Self = Self(0xf4);
    pub const GL_CIRCULAR_CW_ARC_TO_NV: Self = Self(0xfa);
    pub const GL_RELATIVE_SMOOTH_CUBIC_CURVE_TO_NV: Self = Self(0x11);
    pub const GL_RELATIVE_VERTICAL_LINE_TO_NV: Self = Self(0x09);
    pub const GL_CONIC_CURVE_TO_NV: Self = Self(0x1a);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GraphicsResetStatus(pub(crate) u64);
impl GraphicsResetStatus {
    pub const GL_UNKNOWN_CONTEXT_RESET: Self = Self(0x8255);
    pub const GL_NO_ERROR: Self = Self(0);
    pub const GL_GUILTY_CONTEXT_RESET: Self = Self(0x8253);
    pub const GL_INNOCENT_CONTEXT_RESET: Self = Self(0x8254);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TextureParameterName(pub(crate) u64);
impl TextureParameterName {
    pub const GL_TEXTURE_SWIZZLE_R: Self = Self(0x8e42);
    pub const GL_TEXTURE_4DSIZE_SGIS: Self = Self(0x8136);
    pub const GL_TEXTURE_UNNORMALIZED_COORDINATES_ARM: Self = Self(0x8f6a);
    pub const GL_TEXTURE_PRIORITY_EXT: Self = Self(0x8066);
    pub const GL_SHARPEN_TEXTURE_FUNC_POINTS_SGIS: Self = Self(0x80b0);
    pub const GL_TEXTURE_COMPARE_OPERATOR_SGIX: Self = Self(0x819b);
    pub const GL_TEXTURE_CLIPMAP_OFFSET_SGIX: Self = Self(0x8173);
    pub const GL_TEXTURE_CLIPMAP_FRAME_SGIX: Self = Self(0x8172);
    pub const GL_TEXTURE_MAX_CLAMP_T_SGIX: Self = Self(0x836a);
    pub const GL_TEXTURE_LOD_BIAS_S_SGIX: Self = Self(0x818e);
    pub const GL_TEXTURE_BORDER_COLOR_NV: Self = Self(0x1004);
    pub const GL_TEXTURE_MAX_LOD: Self = Self(0x813b);
    pub const GL_TEXTURE_CLIPMAP_CENTER_SGIX: Self = Self(0x8171);
    pub const GL_POST_TEXTURE_FILTER_BIAS_SGIX: Self = Self(0x8179);
    pub const GL_TEXTURE_LOD_BIAS: Self = Self(0x8501);
    pub const GL_TEXTURE_DEPTH_EXT: Self = Self(0x8071);
    pub const GL_TEXTURE_MAX_CLAMP_S_SGIX: Self = Self(0x8369);
    pub const GL_TEXTURE_MAG_FILTER: Self = Self(0x2800);
    pub const GL_TEXTURE_COMPARE_FUNC: Self = Self(0x884d);
    pub const GL_TEXTURE_SWIZZLE_A: Self = Self(0x8e45);
    pub const GL_SHADOW_AMBIENT_SGIX: Self = Self(0x80bf);
    pub const GL_DETAIL_TEXTURE_LEVEL_SGIS: Self = Self(0x809a);
    pub const GL_TEXTURE_MIN_LOD_SGIS: Self = Self(0x813a);
    pub const GL_TEXTURE_RED_SIZE: Self = Self(0x805c);
    pub const GL_TEXTURE_WRAP_R: Self = Self(0x8072);
    pub const GL_TEXTURE_GEQUAL_R_SGIX: Self = Self(0x819d);
    pub const GL_TEXTURE_RESIDENT: Self = Self(0x8067);
    pub const GL_TEXTURE_INTERNAL_FORMAT: Self = Self(0x1003);
    pub const GL_TEXTURE_COMPARE_SGIX: Self = Self(0x819a);
    pub const GL_TEXTURE_MAX_LEVEL: Self = Self(0x813d);
    pub const GL_TEXTURE_CLIPMAP_LOD_OFFSET_SGIX: Self = Self(0x8175);
    pub const GL_TEXTURE_ALPHA_SIZE: Self = Self(0x805f);
    pub const GL_TEXTURE_MIN_FILTER: Self = Self(0x2801);
    pub const GL_TEXTURE_MAX_ANISOTROPY: Self = Self(0x84fe);
    pub const GL_DEPTH_STENCIL_TEXTURE_MODE: Self = Self(0x90ea);
    pub const GL_GENERATE_MIPMAP_SGIS: Self = Self(0x8191);
    pub const GL_TEXTURE_MAX_CLAMP_R_SGIX: Self = Self(0x836b);
    pub const GL_TEXTURE_WRAP_R_EXT: Self = Self(0x8072);
    pub const GL_TEXTURE_LEQUAL_R_SGIX: Self = Self(0x819c);
    pub const GL_TEXTURE_BASE_LEVEL: Self = Self(0x813c);
    pub const GL_TEXTURE_LUMINANCE_SIZE: Self = Self(0x8060);
    pub const GL_DUAL_TEXTURE_SELECT_SGIS: Self = Self(0x8124);
    pub const GL_TEXTURE_MIN_LOD: Self = Self(0x813a);
    pub const GL_TEXTURE_GREEN_SIZE: Self = Self(0x805d);
    pub const GL_TEXTURE_SWIZZLE_B: Self = Self(0x8e44);
    pub const GL_TEXTURE_CLIPMAP_DEPTH_SGIX: Self = Self(0x8176);
    pub const GL_TEXTURE_LOD_BIAS_T_SGIX: Self = Self(0x818f);
    pub const GL_TEXTURE_COMPARE_MODE: Self = Self(0x884c);
    pub const GL_DETAIL_TEXTURE_MODE_SGIS: Self = Self(0x809b);
    pub const GL_TEXTURE_FILTER4_SIZE_SGIS: Self = Self(0x8147);
    pub const GL_TEXTURE_BLUE_SIZE: Self = Self(0x805e);
    pub const GL_TEXTURE_WRAP_S: Self = Self(0x2802);
    pub const GL_DETAIL_TEXTURE_FUNC_POINTS_SGIS: Self = Self(0x809c);
    pub const GL_TEXTURE_CLIPMAP_VIRTUAL_DEPTH_SGIX: Self = Self(0x8174);
    pub const GL_TEXTURE_SWIZZLE_RGBA: Self = Self(0x8e46);
    pub const GL_TEXTURE_MAX_LOD_SGIS: Self = Self(0x813b);
    pub const GL_TEXTURE_HEIGHT: Self = Self(0x1001);
    pub const GL_TEXTURE_BASE_LEVEL_SGIS: Self = Self(0x813c);
    pub const GL_TEXTURE_FOVEATED_CUTOFF_DENSITY_QCOM: Self = Self(0x96a0);
    pub const GL_TEXTURE_WRAP_T: Self = Self(0x2803);
    pub const GL_TEXTURE_WRAP_Q_SGIS: Self = Self(0x8137);
    pub const GL_TEXTURE_BORDER_COLOR: Self = Self(0x1004);
    pub const GL_TEXTURE_WIDTH: Self = Self(0x1000);
    pub const GL_TEXTURE_BORDER: Self = Self(0x1005);
    pub const GL_TEXTURE_LOD_BIAS_R_SGIX: Self = Self(0x8190);
    pub const GL_TEXTURE_TILING_EXT: Self = Self(0x9580);
    pub const GL_TEXTURE_INTENSITY_SIZE: Self = Self(0x8061);
    pub const GL_TEXTURE_WRAP_R_OES: Self = Self(0x8072);
    pub const GL_QUAD_TEXTURE_SELECT_SGIS: Self = Self(0x8125);
    pub const GL_TEXTURE_SWIZZLE_G: Self = Self(0x8e43);
    pub const GL_GENERATE_MIPMAP: Self = Self(0x8191);
    pub const GL_TEXTURE_COMPONENTS: Self = Self(0x1003);
    pub const GL_POST_TEXTURE_FILTER_SCALE_SGIX: Self = Self(0x817a);
    pub const GL_TEXTURE_PRIORITY: Self = Self(0x8066);
    pub const GL_TEXTURE_MAX_LEVEL_SGIS: Self = Self(0x813d);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct UniformPName(pub(crate) u64);
impl UniformPName {
    pub const GL_UNIFORM_OFFSET: Self = Self(0x8a3b);
    pub const GL_UNIFORM_NAME_LENGTH: Self = Self(0x8a39);
    pub const GL_UNIFORM_SIZE: Self = Self(0x8a38);
    pub const GL_UNIFORM_BLOCK_INDEX: Self = Self(0x8a3a);
    pub const GL_UNIFORM_TYPE: Self = Self(0x8a37);
    pub const GL_UNIFORM_MATRIX_STRIDE: Self = Self(0x8a3d);
    pub const GL_UNIFORM_ATOMIC_COUNTER_BUFFER_INDEX: Self = Self(0x92da);
    pub const GL_UNIFORM_ARRAY_STRIDE: Self = Self(0x8a3c);
    pub const GL_UNIFORM_IS_ROW_MAJOR: Self = Self(0x8a3e);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StencilOp(pub(crate) u64);
impl StencilOp {
    pub const GL_REPLACE: Self = Self(0x1e01);
    pub const GL_DECR_WRAP: Self = Self(0x8508);
    pub const GL_ZERO: Self = Self(0);
    pub const GL_INVERT: Self = Self(0x150a);
    pub const GL_KEEP: Self = Self(0x1e00);
    pub const GL_INCR: Self = Self(0x1e02);
    pub const GL_INCR_WRAP: Self = Self(0x8507);
    pub const GL_DECR: Self = Self(0x1e03);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DataTypeEXT(pub(crate) u64);
impl DataTypeEXT {
    pub const GL_SCALAR_EXT: Self = Self(0x87be);
    pub const GL_VECTOR_EXT: Self = Self(0x87bf);
    pub const GL_MATRIX_EXT: Self = Self(0x87c0);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TextureGenMode(pub(crate) u64);
impl TextureGenMode {
    pub const GL_SPHERE_MAP: Self = Self(0x2402);
    pub const GL_OBJECT_DISTANCE_TO_POINT_SGIS: Self = Self(0x81f1);
    pub const GL_EYE_DISTANCE_TO_LINE_SGIS: Self = Self(0x81f2);
    pub const GL_OBJECT_DISTANCE_TO_LINE_SGIS: Self = Self(0x81f3);
    pub const GL_EYE_LINEAR: Self = Self(0x2400);
    pub const GL_OBJECT_LINEAR: Self = Self(0x2401);
    pub const GL_EYE_DISTANCE_TO_POINT_SGIS: Self = Self(0x81f0);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AccumOp(pub(crate) u64);
impl AccumOp {
    pub const GL_MULT: Self = Self(0x0103);
    pub const GL_ACCUM: Self = Self(0x0100);
    pub const GL_LOAD: Self = Self(0x0101);
    pub const GL_RETURN: Self = Self(0x0102);
    pub const GL_ADD: Self = Self(0x0104);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GetTextureParameter(pub(crate) u64);
impl GetTextureParameter {
    pub const GL_SURFACE_COMPRESSION_EXT: Self = Self(0x96c0);
    pub const GL_TEXTURE_CLIPMAP_FRAME_SGIX: Self = Self(0x8172);
    pub const GL_SHADOW_AMBIENT_SGIX: Self = Self(0x80bf);
    pub const GL_TEXTURE_WRAP_S: Self = Self(0x2802);
    pub const GL_TEXTURE_RESIDENT: Self = Self(0x8067);
    pub const GL_TEXTURE_LOD_BIAS_T_SGIX: Self = Self(0x818f);
    pub const GL_TEXTURE_WRAP_T: Self = Self(0x2803);
    pub const GL_TEXTURE_UNNORMALIZED_COORDINATES_ARM: Self = Self(0x8f6a);
    pub const GL_TEXTURE_LEQUAL_R_SGIX: Self = Self(0x819c);
    pub const GL_TEXTURE_CLIPMAP_DEPTH_SGIX: Self = Self(0x8176);
    pub const GL_TEXTURE_4DSIZE_SGIS: Self = Self(0x8136);
    pub const GL_TEXTURE_CLIPMAP_LOD_OFFSET_SGIX: Self = Self(0x8175);
    pub const GL_POST_TEXTURE_FILTER_SCALE_SGIX: Self = Self(0x817a);
    pub const GL_TEXTURE_WIDTH: Self = Self(0x1000);
    pub const GL_TEXTURE_BLUE_SIZE: Self = Self(0x805e);
    pub const GL_TEXTURE_WRAP_R_EXT: Self = Self(0x8072);
    pub const GL_TEXTURE_INTERNAL_FORMAT: Self = Self(0x1003);
    pub const GL_TEXTURE_INTENSITY_SIZE: Self = Self(0x8061);
    pub const GL_TEXTURE_ALPHA_SIZE: Self = Self(0x805f);
    pub const GL_SHARPEN_TEXTURE_FUNC_POINTS_SGIS: Self = Self(0x80b0);
    pub const GL_TEXTURE_LUMINANCE_SIZE: Self = Self(0x8060);
    pub const GL_TEXTURE_MAX_LOD_SGIS: Self = Self(0x813b);
    pub const GL_TEXTURE_MAX_LEVEL_SGIS: Self = Self(0x813d);
    pub const GL_TEXTURE_CLIPMAP_OFFSET_SGIX: Self = Self(0x8173);
    pub const GL_GENERATE_MIPMAP_SGIS: Self = Self(0x8191);
    pub const GL_TEXTURE_COMPARE_SGIX: Self = Self(0x819a);
    pub const GL_TEXTURE_PRIORITY: Self = Self(0x8066);
    pub const GL_TEXTURE_GREEN_SIZE: Self = Self(0x805d);
    pub const GL_TEXTURE_BASE_LEVEL_SGIS: Self = Self(0x813c);
    pub const GL_TEXTURE_GEQUAL_R_SGIX: Self = Self(0x819d);
    pub const GL_TEXTURE_MAX_CLAMP_S_SGIX: Self = Self(0x8369);
    pub const GL_TEXTURE_MAX_CLAMP_R_SGIX: Self = Self(0x836b);
    pub const GL_TEXTURE_BORDER: Self = Self(0x1005);
    pub const GL_TEXTURE_MAG_FILTER: Self = Self(0x2800);
    pub const GL_QUAD_TEXTURE_SELECT_SGIS: Self = Self(0x8125);
    pub const GL_TEXTURE_LOD_BIAS_R_SGIX: Self = Self(0x8190);
    pub const GL_TEXTURE_RED_SIZE: Self = Self(0x805c);
    pub const GL_TEXTURE_COMPONENTS: Self = Self(0x1003);
    pub const GL_TEXTURE_FILTER4_SIZE_SGIS: Self = Self(0x8147);
    pub const GL_TEXTURE_COMPARE_OPERATOR_SGIX: Self = Self(0x819b);
    pub const GL_DETAIL_TEXTURE_LEVEL_SGIS: Self = Self(0x809a);
    pub const GL_TEXTURE_CLIPMAP_VIRTUAL_DEPTH_SGIX: Self = Self(0x8174);
    pub const GL_DETAIL_TEXTURE_MODE_SGIS: Self = Self(0x809b);
    pub const GL_TEXTURE_MIN_LOD_SGIS: Self = Self(0x813a);
    pub const GL_TEXTURE_HEIGHT: Self = Self(0x1001);
    pub const GL_TEXTURE_DEPTH_EXT: Self = Self(0x8071);
    pub const GL_DETAIL_TEXTURE_FUNC_POINTS_SGIS: Self = Self(0x809c);
    pub const GL_TEXTURE_WRAP_Q_SGIS: Self = Self(0x8137);
    pub const GL_TEXTURE_BORDER_COLOR: Self = Self(0x1004);
    pub const GL_DUAL_TEXTURE_SELECT_SGIS: Self = Self(0x8124);
    pub const GL_TEXTURE_MIN_FILTER: Self = Self(0x2801);
    pub const GL_TEXTURE_CLIPMAP_CENTER_SGIX: Self = Self(0x8171);
    pub const GL_POST_TEXTURE_FILTER_BIAS_SGIX: Self = Self(0x8179);
    pub const GL_TEXTURE_LOD_BIAS_S_SGIX: Self = Self(0x818e);
    pub const GL_TEXTURE_BORDER_COLOR_NV: Self = Self(0x1004);
    pub const GL_TEXTURE_MAX_CLAMP_T_SGIX: Self = Self(0x836a);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SeparableTarget(pub(crate) u64);
impl SeparableTarget {
    pub const GL_SEPARABLE_2D: Self = Self(0x8012);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ProgramParameterPName(pub(crate) u64);
impl ProgramParameterPName {
    pub const GL_PROGRAM_SEPARABLE: Self = Self(0x8258);
    pub const GL_PROGRAM_BINARY_RETRIEVABLE_HINT: Self = Self(0x8257);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GetConvolutionParameter(pub(crate) u64);
impl GetConvolutionParameter {
    pub const GL_CONVOLUTION_FORMAT_EXT: Self = Self(0x8017);
    pub const GL_MAX_CONVOLUTION_HEIGHT: Self = Self(0x801b);
    pub const GL_CONVOLUTION_FILTER_BIAS_EXT: Self = Self(0x8015);
    pub const GL_CONVOLUTION_BORDER_MODE_EXT: Self = Self(0x8013);
    pub const GL_CONVOLUTION_BORDER_COLOR: Self = Self(0x8154);
    pub const GL_MAX_CONVOLUTION_WIDTH_EXT: Self = Self(0x801a);
    pub const GL_CONVOLUTION_FILTER_SCALE: Self = Self(0x8014);
    pub const GL_CONVOLUTION_FORMAT: Self = Self(0x8017);
    pub const GL_CONVOLUTION_WIDTH_EXT: Self = Self(0x8018);
    pub const GL_CONVOLUTION_FILTER_BIAS: Self = Self(0x8015);
    pub const GL_CONVOLUTION_BORDER_MODE: Self = Self(0x8013);
    pub const GL_CONVOLUTION_WIDTH: Self = Self(0x8018);
    pub const GL_MAX_CONVOLUTION_WIDTH: Self = Self(0x801a);
    pub const GL_MAX_CONVOLUTION_HEIGHT_EXT: Self = Self(0x801b);
    pub const GL_CONVOLUTION_HEIGHT_EXT: Self = Self(0x8019);
    pub const GL_CONVOLUTION_FILTER_SCALE_EXT: Self = Self(0x8014);
    pub const GL_CONVOLUTION_HEIGHT: Self = Self(0x8019);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LightTexturePNameEXT(pub(crate) u64);
impl LightTexturePNameEXT {
    pub const GL_ATTENUATION_EXT: Self = Self(0x834d);
    pub const GL_SHADOW_ATTENUATION_EXT: Self = Self(0x834e);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ProgramStringProperty(pub(crate) u64);
impl ProgramStringProperty {
    pub const GL_PROGRAM_STRING_ARB: Self = Self(0x8628);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BlitFramebufferFilter(pub(crate) u64);
impl BlitFramebufferFilter {
    pub const GL_NEAREST: Self = Self(0x2600);
    pub const GL_LINEAR: Self = Self(0x2601);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DebugType(pub(crate) u64);
impl DebugType {
    pub const GL_DONT_CARE: Self = Self(0x1100);
    pub const GL_DEBUG_TYPE_PERFORMANCE: Self = Self(0x8250);
    pub const GL_DEBUG_TYPE_DEPRECATED_BEHAVIOR: Self = Self(0x824d);
    pub const GL_DEBUG_TYPE_PORTABILITY: Self = Self(0x824f);
    pub const GL_DEBUG_TYPE_UNDEFINED_BEHAVIOR: Self = Self(0x824e);
    pub const GL_DEBUG_TYPE_OTHER: Self = Self(0x8251);
    pub const GL_DEBUG_TYPE_POP_GROUP: Self = Self(0x826a);
    pub const GL_DEBUG_TYPE_PUSH_GROUP: Self = Self(0x8269);
    pub const GL_DEBUG_TYPE_MARKER: Self = Self(0x8268);
    pub const GL_DEBUG_TYPE_ERROR: Self = Self(0x824c);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ProgramPropertyARB(pub(crate) u64);
impl ProgramPropertyARB {
    pub const GL_ACTIVE_ATOMIC_COUNTER_BUFFERS: Self = Self(0x92d9);
    pub const GL_GEOMETRY_VERTICES_OUT: Self = Self(0x8916);
    pub const GL_ACTIVE_UNIFORM_BLOCK_MAX_NAME_LENGTH: Self = Self(0x8a35);
    pub const GL_COMPUTE_WORK_GROUP_SIZE: Self = Self(0x8267);
    pub const GL_PROGRAM_BINARY_LENGTH: Self = Self(0x8741);
    pub const GL_TRANSFORM_FEEDBACK_BUFFER_MODE: Self = Self(0x8c7f);
    pub const GL_ACTIVE_UNIFORM_BLOCKS: Self = Self(0x8a36);
    pub const GL_TRANSFORM_FEEDBACK_VARYING_MAX_LENGTH: Self = Self(0x8c76);
    pub const GL_ACTIVE_UNIFORMS: Self = Self(0x8b86);
    pub const GL_ACTIVE_UNIFORM_MAX_LENGTH: Self = Self(0x8b87);
    pub const GL_GEOMETRY_OUTPUT_TYPE: Self = Self(0x8918);
    pub const GL_GEOMETRY_INPUT_TYPE: Self = Self(0x8917);
    pub const GL_VALIDATE_STATUS: Self = Self(0x8b83);
    pub const GL_LINK_STATUS: Self = Self(0x8b82);
    pub const GL_INFO_LOG_LENGTH: Self = Self(0x8b84);
    pub const GL_DELETE_STATUS: Self = Self(0x8b80);
    pub const GL_ATTACHED_SHADERS: Self = Self(0x8b85);
    pub const GL_ACTIVE_ATTRIBUTES: Self = Self(0x8b89);
    pub const GL_ACTIVE_ATTRIBUTE_MAX_LENGTH: Self = Self(0x8b8a);
    pub const GL_TRANSFORM_FEEDBACK_VARYINGS: Self = Self(0x8c83);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ClampColorTargetARB(pub(crate) u64);
impl ClampColorTargetARB {
    pub const GL_CLAMP_READ_COLOR: Self = Self(0x891c);
    pub const GL_CLAMP_READ_COLOR_ARB: Self = Self(0x891c);
    pub const GL_CLAMP_FRAGMENT_COLOR_ARB: Self = Self(0x891b);
    pub const GL_CLAMP_VERTEX_COLOR_ARB: Self = Self(0x891a);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ConvolutionParameterEXT(pub(crate) u64);
impl ConvolutionParameterEXT {
    pub const GL_CONVOLUTION_FILTER_SCALE: Self = Self(0x8014);
    pub const GL_CONVOLUTION_FILTER_BIAS: Self = Self(0x8015);
    pub const GL_CONVOLUTION_BORDER_MODE: Self = Self(0x8013);
    pub const GL_CONVOLUTION_FILTER_BIAS_EXT: Self = Self(0x8015);
    pub const GL_CONVOLUTION_FILTER_SCALE_EXT: Self = Self(0x8014);
    pub const GL_CONVOLUTION_BORDER_MODE_EXT: Self = Self(0x8013);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SecondaryColorPointerTypeIBM(pub(crate) u64);
impl SecondaryColorPointerTypeIBM {
    pub const GL_SHORT: Self = Self(0x1402);
    pub const GL_FLOAT: Self = Self(0x1406);
    pub const GL_DOUBLE: Self = Self(0x140a);
    pub const GL_INT: Self = Self(0x1404);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VertexAttribEnumNV(pub(crate) u64);
impl VertexAttribEnumNV {
    pub const GL_PROGRAM_PARAMETER_NV: Self = Self(0x8644);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PolygonMode(pub(crate) u64);
impl PolygonMode {
    pub const GL_LINE: Self = Self(0x1b01);
    pub const GL_FILL: Self = Self(0x1b02);
    pub const GL_POINT: Self = Self(0x1b00);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TextureGenParameter(pub(crate) u64);
impl TextureGenParameter {
    pub const GL_OBJECT_LINE_SGIS: Self = Self(0x81f7);
    pub const GL_OBJECT_PLANE: Self = Self(0x2501);
    pub const GL_TEXTURE_GEN_MODE: Self = Self(0x2500);
    pub const GL_EYE_LINE_SGIS: Self = Self(0x81f6);
    pub const GL_OBJECT_POINT_SGIS: Self = Self(0x81f5);
    pub const GL_EYE_PLANE: Self = Self(0x2502);
    pub const GL_EYE_POINT_SGIS: Self = Self(0x81f4);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TextureMinFilter(pub(crate) u64);
impl TextureMinFilter {
    pub const GL_NEAREST: Self = Self(0x2600);
    pub const GL_LINEAR_MIPMAP_NEAREST: Self = Self(0x2701);
    pub const GL_NEAREST_MIPMAP_LINEAR: Self = Self(0x2702);
    pub const GL_NEAREST_CLIPMAP_NEAREST_SGIX: Self = Self(0x844d);
    pub const GL_PIXEL_TEX_GEN_Q_FLOOR_SGIX: Self = Self(0x8186);
    pub const GL_NEAREST_MIPMAP_NEAREST: Self = Self(0x2700);
    pub const GL_LINEAR_MIPMAP_LINEAR: Self = Self(0x2703);
    pub const GL_FILTER4_SGIS: Self = Self(0x8146);
    pub const GL_PIXEL_TEX_GEN_Q_CEILING_SGIX: Self = Self(0x8184);
    pub const GL_PIXEL_TEX_GEN_Q_ROUND_SGIX: Self = Self(0x8185);
    pub const GL_LINEAR: Self = Self(0x2601);
    pub const GL_NEAREST_CLIPMAP_LINEAR_SGIX: Self = Self(0x844e);
    pub const GL_LINEAR_CLIPMAP_LINEAR_SGIX: Self = Self(0x8170);
    pub const GL_LINEAR_CLIPMAP_NEAREST_SGIX: Self = Self(0x844f);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ProgramTarget(pub(crate) u64);
impl ProgramTarget {
    pub const GL_FRAGMENT_PROGRAM_ARB: Self = Self(0x8804);
    pub const GL_TEXT_FRAGMENT_SHADER_ATI: Self = Self(0x8200);
    pub const GL_VERTEX_PROGRAM_ARB: Self = Self(0x8620);
    pub const GL_TESS_CONTROL_PROGRAM_NV: Self = Self(0x891e);
    pub const GL_GEOMETRY_PROGRAM_NV: Self = Self(0x8c26);
    pub const GL_TESS_EVALUATION_PROGRAM_NV: Self = Self(0x891f);
    pub const GL_COMPUTE_PROGRAM_NV: Self = Self(0x90fb);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FragmentLightNameSGIX(pub(crate) u64);
impl FragmentLightNameSGIX {
    pub const GL_FRAGMENT_LIGHT7_SGIX: Self = Self(0x8413);
    pub const GL_FRAGMENT_LIGHT6_SGIX: Self = Self(0x8412);
    pub const GL_FRAGMENT_LIGHT5_SGIX: Self = Self(0x8411);
    pub const GL_FRAGMENT_LIGHT2_SGIX: Self = Self(0x840e);
    pub const GL_FRAGMENT_LIGHT3_SGIX: Self = Self(0x840f);
    pub const GL_FRAGMENT_LIGHT4_SGIX: Self = Self(0x8410);
    pub const GL_FRAGMENT_LIGHT1_SGIX: Self = Self(0x840d);
    pub const GL_FRAGMENT_LIGHT0_SGIX: Self = Self(0x840c);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DepthFunction(pub(crate) u64);
impl DepthFunction {
    pub const GL_NOTEQUAL: Self = Self(0x0205);
    pub const GL_GEQUAL: Self = Self(0x0206);
    pub const GL_LEQUAL: Self = Self(0x0203);
    pub const GL_LESS: Self = Self(0x0201);
    pub const GL_GREATER: Self = Self(0x0204);
    pub const GL_ALWAYS: Self = Self(0x0207);
    pub const GL_EQUAL: Self = Self(0x0202);
    pub const GL_NEVER: Self = Self(0x0200);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MapAttribParameterNV(pub(crate) u64);
impl MapAttribParameterNV {
    pub const GL_MAP_ATTRIB_V_ORDER_NV: Self = Self(0x86c4);
    pub const GL_MAP_ATTRIB_U_ORDER_NV: Self = Self(0x86c3);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SeparableTargetEXT(pub(crate) u64);
impl SeparableTargetEXT {
    pub const GL_SEPARABLE_2D: Self = Self(0x8012);
    pub const GL_SEPARABLE_2D_EXT: Self = Self(0x8012);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SyncBehaviorFlags(pub(crate) u64);
impl SyncBehaviorFlags {
    pub const GL_NONE: Self = Self(0);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PathStringFormat(pub(crate) u64);
impl PathStringFormat {
    pub const GL_PATH_FORMAT_PS_NV: Self = Self(0x9071);
    pub const GL_PATH_FORMAT_SVG_NV: Self = Self(0x9070);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ConvolutionTargetEXT(pub(crate) u64);
impl ConvolutionTargetEXT {
    pub const GL_CONVOLUTION_1D_EXT: Self = Self(0x8010);
    pub const GL_CONVOLUTION_2D: Self = Self(0x8011);
    pub const GL_CONVOLUTION_1D: Self = Self(0x8010);
    pub const GL_CONVOLUTION_2D_EXT: Self = Self(0x8011);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LightTextureModeEXT(pub(crate) u64);
impl LightTextureModeEXT {
    pub const GL_FRAGMENT_MATERIAL_EXT: Self = Self(0x8349);
    pub const GL_FRAGMENT_NORMAL_EXT: Self = Self(0x834a);
    pub const GL_FRAGMENT_COLOR_EXT: Self = Self(0x834c);
    pub const GL_FRAGMENT_DEPTH_EXT: Self = Self(0x8452);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PathColor(pub(crate) u64);
impl PathColor {
    pub const GL_PRIMARY_COLOR: Self = Self(0x8577);
    pub const GL_SECONDARY_COLOR_NV: Self = Self(0x852d);
    pub const GL_PRIMARY_COLOR_NV: Self = Self(0x852c);
}

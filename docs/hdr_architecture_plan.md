# HDR integration notes

## Current compositor architecture (survey)
- **Entry point & event loop**: `src/lib.rs` sets up logging, Wayland display, compositor state, and runs the `calloop` event loop, scheduling renders and animations via `backend::init_backend_auto` and `event_loop.run`.
- **Output configuration / KMS**: `src/backend/kms/mod.rs` manages DRM devices, session, and output mappings. `apply_config_for_outputs` chooses connectors/CRTCs and refreshes surfaces per output based on config and availability.
- **DRM surface & dmabuf feedback**: `src/backend/kms/surface/mod.rs` owns per-output rendering threads and builds dmabuf feedback by intersecting render/scanout format sets for primary and overlay planes, ensuring scanout formats are renderable.
- **Renderer / composition**: `src/backend/render/mod.rs` wraps Smithay’s `MultiRenderer`/`GlowRenderer` stack, builds render elements (Wayland surfaces, textures, damage tracking), and drives postprocessing shaders.
- **Pixel-format selection**: When constructing `GbmDrmOutputManager`, the KMS device advertises scanout formats including 10-bit `Abgr2101010`/`Argb2101010` alongside 8-bit formats, paired with GBM allocators and framebuffer exporter. Dmabuf feedback prefers the intersection of render node formats with target/plane capabilities.
- **Wayland protocol glue**: `src/state.rs` wires Smithay protocol states (compositor, dmabuf, output management, etc.) and tracks dmabuf feedback / surface primary scanout outputs. Wayland handler modules under `src/wayland/handlers/` implement protocol-specific behavior.

## Proposed HDR architecture (high level)
- **Per-output capabilities**: Extend KMS output metadata to record parsed EDID colorimetry (EOTF support, primaries, luminance) and KMS HDR properties (`HDR_OUTPUT_METADATA`, supported deep-color formats). Store on `Surface`/`Output` and expose through Wayland color-management-v1.
- **Per-surface color state**: Attach color-space/EOTF + optional HDR metadata to each surface (Wayland role), derived from the color-management-v1 requests. Track in surface data and propagate into render element creation.
- **Rendering space**: Move composition to a linear, wide-gamut fp16 render target (or equivalent) in the existing renderer module, performing per-surface sampling with transfer-function conversion into linear space before blending.
- **Tone mapping**: Add a configurable tone-mapper (e.g., Hable/Reinhard) used for SDR→HDR on HDR outputs and HDR→SDR fallback. Keep tunables per-output (min/peak luminance) sourced from EDID or overrides.
- **Output programming**: During KMS commit, choose HDR-capable plane formats (10/12-bit) when the output is HDR-enabled. Write HDR static metadata based on output characteristics and active scene, and fall back to SDR if unsupported.
- **Wayland protocol support**: Integrate Smithay’s color-management-v1 bindings to advertise output color spaces (sRGB, Display-P3, Rec.2020) and EOTFs (PQ/HLG), accept per-surface color intents, and bind HDR metadata flow into surface state.
- **Policy**: Default per-output mode to Auto (enable HDR when hardware + pipeline + content permit). Tone-map SDR on HDR outputs; tone-map HDR down to SDR outputs. Provide env/config toggles and debug logging for capability detection and mode switches.

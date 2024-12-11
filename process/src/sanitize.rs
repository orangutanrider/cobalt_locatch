use locatch_lib::*;

pub async fn tunnels_sanitize(tunnels: &mut Vec<TunnelResponse>) {
    for tunnel in tunnels.iter_mut() { // par SIMD possible?
        sanitize_filename::sanitize(&mut tunnel.filename);
    }
}

pub async fn pickers_sanitize(pickers: &mut Vec<PickerResponse>) {
    for picker in pickers.iter_mut() { // par SIMD possible?
        let Some(audio_filename) = &mut picker.audio_filename else {
            continue;
        };

        sanitize_filename::sanitize(audio_filename);
    }
}
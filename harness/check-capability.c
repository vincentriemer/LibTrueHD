/* Run against a built slice to verify exact decoding is available, not just source flags. */
#include "truehd_atmos.h"
#include <assert.h>
int main(void) {
    TrueHDAtmosDecoder *decoder = truehd_atmos_decoder_create(TRUEHD_ATMOS_PRESENTATION_HIGHEST);
    assert(decoder);
    assert(truehd_atmos_decoder_set_ignore_interpolation(decoder, false) == TRUEHD_ATMOS_OK);
    truehd_atmos_decoder_destroy(decoder);
    return 0;
}

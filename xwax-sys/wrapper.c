#include "wrapper.h"

struct timecode_def* _timecoder_get_definition(struct timecoder *tc)
{
    return timecoder_get_definition(tc);
}

double _timecoder_get_pitch(struct timecoder *tc)
{
    return timecoder_get_pitch(tc);
}

unsigned int _timecoder_get_safe(struct timecoder *tc)
{
    return timecoder_get_safe(tc);
}

double _timecoder_get_resolution(struct timecoder *tc)
{
    return timecoder_get_resolution(tc);
}

double _timecoder_revs_per_sec(struct timecoder *tc)
{
    return timecoder_revs_per_sec(tc);
}

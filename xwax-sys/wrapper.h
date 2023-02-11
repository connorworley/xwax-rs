#include <stddef.h>
#include "timecoder.h"

// Re-export some static inline functions from timecoder.h
struct timecode_def* _timecoder_get_definition(struct timecoder *tc);
double _timecoder_get_pitch(struct timecoder *tc);
unsigned int _timecoder_get_safe(struct timecoder *tc);
double _timecoder_get_resolution(struct timecoder *tc);
double _timecoder_revs_per_sec(struct timecoder *tc);
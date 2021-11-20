#include <stdio.h>
#include <stdlib.h>
#include <libavutil/error.h>
#include <libavutil/avutil.h>

int SYS_EAGAIN() {
    return EAGAIN;
}

int SYS_AVERROR(int code) {
    return AVERROR(code);
}

int64_t SYS_AV_NOPTS_VALUE() {
    return AV_NOPTS_VALUE;
}

int SYS_AV_ERROR_MAX_STRING_SIZE() {
    return AV_ERROR_MAX_STRING_SIZE;
}

int SYS_AVERROR_BSF_NOT_FOUND() {
    return AVERROR_BSF_NOT_FOUND;
}

int SYS_AVERROR_BUG() {
    return AVERROR_BUG;
}

int SYS_AVERROR_BUFFER_TOO_SMALL() {
    return AVERROR_BUFFER_TOO_SMALL;
}

int SYS_AVERROR_DECODER_NOT_FOUND() {
    return AVERROR_DECODER_NOT_FOUND;
}

int SYS_AVERROR_DEMUXER_NOT_FOUND() {
    return AVERROR_DEMUXER_NOT_FOUND;
}

int SYS_AVERROR_ENCODER_NOT_FOUND() {
    return AVERROR_ENCODER_NOT_FOUND;
}

int SYS_AVERROR_EOF() {
    return AVERROR_EOF;
}

int SYS_AVERROR_EXIT() {
    return AVERROR_EXIT;
}

int SYS_AVERROR_EXTERNAL() {
    return AVERROR_EXTERNAL;
}

int SYS_AVERROR_FILTER_NOT_FOUND() {
    return AVERROR_FILTER_NOT_FOUND;
}

int SYS_AVERROR_INVALIDDATA() {
    return AVERROR_INVALIDDATA;
}

int SYS_AVERROR_MUXER_NOT_FOUND() {
    return AVERROR_MUXER_NOT_FOUND;
}

int SYS_AVERROR_OPTION_NOT_FOUND() {
    return AVERROR_OPTION_NOT_FOUND;
}

int SYS_AVERROR_PATCHWELCOME() {
    return AVERROR_PATCHWELCOME;
}

int SYS_AVERROR_PROTOCOL_NOT_FOUND() {
    return AVERROR_PROTOCOL_NOT_FOUND;
}

int SYS_AVERROR_STREAM_NOT_FOUND() {
    return AVERROR_STREAM_NOT_FOUND;
}

size_t SYS_FFMIN(size_t a, size_t b) {
    return FFMIN(a, b);
}
static const char norm_fg[] = "#78ddea";
static const char norm_bg[] = "#110f1e";
static const char norm_border[] = "#549aa3";

static const char sel_fg[] = "#78ddea";
static const char sel_bg[] = "#076D8E";
static const char sel_border[] = "#78ddea";

static const char urg_fg[] = "#78ddea";
static const char urg_bg[] = "#C14039";
static const char urg_border[] = "#C14039";

static const char *colors[][3]      = {
    /*               fg           bg         border                         */
    [SchemeNorm] = { norm_fg,     norm_bg,   norm_border }, // unfocused wins
    [SchemeSel]  = { sel_fg,      sel_bg,    sel_border },  // the focused win
    [SchemeUrg] =  { urg_fg,      urg_bg,    urg_border },
};

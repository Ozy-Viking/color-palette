const char *colorname[] = {

  /* 8 normal colors */
  [0] = "#110f1e", /* black   */
  [1] = "#C14039", /* red     */
  [2] = "#076D8E", /* green   */
  [3] = "#5A6592", /* yellow  */
  [4] = "#0794B1", /* blue    */
  [5] = "#5BA5B8", /* magenta */
  [6] = "#03B1CC", /* cyan    */
  [7] = "#78ddea", /* white   */

  /* 8 bright colors */
  [8]  = "#549aa3",  /* black   */
  [9]  = "#C14039",  /* red     */
  [10] = "#076D8E", /* green   */
  [11] = "#5A6592", /* yellow  */
  [12] = "#0794B1", /* blue    */
  [13] = "#5BA5B8", /* magenta */
  [14] = "#03B1CC", /* cyan    */
  [15] = "#78ddea", /* white   */

  /* special colors */
  [256] = "#110f1e", /* background */
  [257] = "#78ddea", /* foreground */
  [258] = "#78ddea",     /* cursor */
};

/* Default colors (colorname index)
 * foreground, background, cursor */
 unsigned int defaultbg = 0;
 unsigned int defaultfg = 257;
 unsigned int defaultcs = 258;
 unsigned int defaultrcs= 258;

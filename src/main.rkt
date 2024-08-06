#lang racket
(require ffi/unsafe
ffi/unsafe/define)
(require plot)
;defines path to the lib
(define base_path (string->path(string-append (path->string(current-directory)) "\\interop.dll")))
;defines a ffi-lib
(define lib (ffi-lib base_path #f))
;defines a constant which interacts with functions in the lib
(define-ffi-definer define-rusty lib)
(define-rusty values (_fun -> _void))
(define-rusty width (_fun -> _int32))
(define-rusty height (_fun -> _int32))
(values)
;other type?
(define-rusty title (_fun -> _string))
(define-rusty option (_fun -> _string))
;name options like plot
(plot-new-window? #t)
(if (or (string=? (option) "plot1" (exit 0)))(
(parameterize ([plot-width    (width)]
                 [plot-height   (height)]
                 [plot-title (title)]
                 [plot-x-label  #f]
                 [plot-y-label  #f])
    (list (plot (function sin (- pi) pi))))
)(print("done")))
;(exit))
;example code from docs
;make a heart?
;parametric 3d
;produce ascii to txt file
;when plot is true then activate
 
# File:Quadrature demodulation.svg

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/File%3AQuadrature_demodulation.svg) | View [other pages](Special_AllPages.xhtml#File_Quadrature_demodulation_svg)

[![Media as described below](../wiki-images/Quadrature_demodulation.svg)](../wiki-images/Quadrature_demodulation.svg)

## Summary

This diagram illustrates quadrature demodulation, used in decoding composite signals to color YUV. More details can be found on the [NTSC video](NTSC_video.xhtml "NTSC video") page. 

The three graphs on the left shows how the Y, U, and V component signals are decoded using the composite signal and a reference carrier wave. 

The polar graph on the right shows the resulting color phase and saturation of the decoded color. 

The Y component (shown in orange) is decoded by low passing or comb filtering the input signal. (shown in blue) 

The U component is decoded by multiplying the composite signal to a reference carrier wave (resulting wave shown in blue.) This signal can then be low passed or comb filtered to obtain the U component (shown in orange). 

The V component is decoded in the same way as the U component, but instead with using another reference carrier wave which is phase offset by 90 degrees. 

This diagram, and [the source code that generates this](https://github.com/Gumball2415/palgen-persune) is distributed under a MIT-0 No Attribution license. 

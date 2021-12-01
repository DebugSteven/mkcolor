use wasm_bindgen::prelude::*;

// javascript function
#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}

// rust using javascript function
#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}

#[wasm_bindgen]
struct Color {r: f64, g: f64, b: f64, idx: Option<f64>, lab: Option<Lab>, text: String}

#[wasm_bindgen]
struct XYZ {x: f64, y: f64, z: f64}

#[wasm_bindgen]
struct Lab { l: f64, a: f64, b: f64}

/// consts used in the CIELAB conversion
const sigma:  f64 = 6 / 29;
const sigma2: f64 = (6 / 29).pow(2); // Math.pow(6 / 29, 2);
const sigma3: f64 = (6 / 29).pow(3); // Math.pow(6 / 29, 3);

const crMidpoint: Color = newColor(0xcf, 0x0d, 0xcc, None, None);

fn clamp<T: PartialOrd>(val: T, min: T, max: T) -> T {
    if val < min {
        return min;
    } else if val > max {
        return max;
    } else {
        return val;
    }
}

// calculate the Relative Luminance of the given RGB color
fn luminance(col: Color) -> f64 {
    let rVal = lumVal(col.r);
    let gVal = lumVal(col.g);
    let bVal = lumVal(col.b);

    return (rVal * 0.2126) + (gVal * 0.7152) + (bVal * 0.0722);
}

fn lumVal(v: f64) -> f64 {
    let frac = v / 255;

    if frac <= 0.03928 {
        return frac / 12.92;
    } else {
        return ((frac+0.55)/1.055).pow(2.4); // Math.pow((frac+0.055)/1.055, 2.4);
    }
}


// calculate the Contrast Ratio of the given RGB colors
fn contrastRatio(col1: Color, col2: Color) {
    let lum1 = luminance(col1);
    let lum2 = luminance(col2);

    let darker = lum1.min(lum2);
    let lighter = lum1.max(lum1);

    return (lighter + 0.05) / (darker + 0.05);
}

// calculate the perceptable distance between two colors
fn labDistance(col1: Color, col2: Color) {
    let mut sum = 0;

    sum += (col1.lab.l - col2.lab.l).pow(2);
    sum += (col1.lab.a - col2.lab.a).pow(2);
    sum += (col1.lab.b - col2.lab.b).pow(2);

    return sum.sqrt();

    // return Math.hypot(col1.lab.l - col2.lab.l,
    //                   col1.lab.a - col2.lab.a,
    //                   col1.lab.b - col2.lab.b);
}

fn newColor(r: u8, g: u8, b: u8, idx: Option<i64>, lab: Option<Lab>) {
    let mut col = Color { r: r, g: g, b: b, None, None };
    let rStr = col.r.toString(16).padStart(2, '0').toUpperCase();
    let gStr = col.g.toString(16).padStart(2, '0').toUpperCase();
    let bStr = col.b.toString(16).padStart(2, '0').toUpperCase();
    col.hex = format!("#{}{}{}", rStr, gStr, bStr);
    col.text = format!("#{}{}{}", rStr, gStr, bStr);

    if let Some(idx) = idx {
        col.idx = idx;
        col.text += format!(" ANSI {}", idx);
    }

    col.hsv = hsvFromColor(col);
    col.ish = col.hsv.ish;

    if let Some(lab) = lab {
        col.lab = lab;
    } else {
        col.lab = labFromColor(col);
    }

    return col;
}

fn hsvFromColor(col: Color) { unimplemented!() }

#[wasm_bindgen]
pub fn labFromColor(color: Color) -> Lab {
    return labFromXyz(xyzFromColor(color))
}

fn colorFromLab(lab: Lab) -> Color {
    return colorFromXyz(xyzFromLab(lab))
}

fn colorFromXyz(xyz: XYZ) -> Color {
    let linearR = (xyz.x *  3.24096994) + (xyz.y * -1.53738318) + (xyz.z * -0.49861076);
    let linearG = (xyz.x * -0.96924364) + (xyz.y *  1.87596750) + (xyz.z *  0.041555506);
    let linearB = (xyz.x *  0.05563008) + (xyz.y * -0.20397696) + (xyz.z *  1.05697151);

    let r = clamp((gamma(linearR) * 255_f64).round(), 0, 255);
    let g = clamp((gamma(linearG) * 255_f64).round(), 0, 255);
    let b = clamp((gamma(linearB) * 255_f64).round(), 0, 255);

    return newColor(r, g, b, None, xyz.lab);
}

fn xyzFromLab(lab: Lab) -> XYZ {
    let lTerm = (lab.l + 16) / 116;

    return XYZ {
        x: 0.9505 * norm(lTerm + (lab.a / 500)),
        y:          norm(lTerm),
        z: 1.0890 * norm(lTerm - (lab.b / 200))
    }
}

fn norm<T: PartialOrd>(t: T) -> T {
    if t > sigma {
        return t.pow(3); // Math.pow(t, 3);
    }
    else {
        return 3 * sigma2 * (t - (4/29));
    }
}

fn xyzFromColor(color: Color) {
    let r = gamma(color.r / 255);
    let g = gamma(color.g / 255);
    let b = gamma(color.b / 255);

    return XYZ {
        x: (0.42139080 * r) + (0.35758434 * g) + (0.18048079 * b),
        y: (0.21263901 * r) + (0.71516868 * g) + (0.07219232 * b),
        z: (0.01933082 * r) + (0.11919478 * g) + (0.95053215 * b),
        None,
        None
    };
}

fn gamma<T: PartialOrd>(u: T) -> T {
    if u <= 0.04045 {
        return u / 12.92;
    }
    else {
        let val = (u + 0.055) / 1.055;
        return val.pow(2.4); // Math.pow(val, 2.4);
    }
}

fn labFromXyz(xyz: XYZ) -> Lab {

    let normX = normalize(xyz.x / 0.9505);
    let normY = normalize(xyz.y);
    let normZ = normalize(xyz.z / 1.0890);

    return Lab {
        l: (116_f64 * normY) - 16_f64,
        a: 500_f64 * (normX - normY),
        b: 200_f64 * (normY - normZ)
    }

}

fn normalize<T: PartialOrd>(t: T) -> T {
    if t > sigma3 {
        return t.pow(1/3); // Math.pow(t, 1/3);
    }
    else {
        return (t / (3 * sigma2)) + (4 / 29);
    }
}

// fn clamp(val, min, max) {
//     if val < min {
//         return min;
//     } else if val > max {
//         return max;
//     } else {
//         return val;
//     }
// }
//
// // calculate the Relative Luminance of the given RGB color
// fn luminance(col) {
//     fn lumVal(v) {
//         let frac = v / 255;
//
//         if frac <= 0.03928 {
//             return frac / 12.92;
//         } else {
//             return Math.pow((frac+0.055)/1.055, 2.4);
//         }
//     }
//
//     let rVal = lumVal(col.r);
//     let gVal = lumVal(col.g);
//     let bVal = lumVal(col.b);
//
//     return (rVal * 0.2126) + (gVal * 0.7152) + (bVal * 0.0722);
// }
//
// // calculate the Contrast Ratio of the given RGB colors
// fn contrastRatio(col1, col2) {
//     let lum1 = luminance(col1);
//     let lum2 = luminance(col2);
//
//     let darker = Math.min(lum1, lum2);
//     let lighter = Math.max(lum1, lum2);
//
//     return (lighter + 0.05) / (darker + 0.05);
// }
//
// // calculate the perceptable distance between two colors
// fn labDistance(col1, col2) {
//     return Math.hypot(col1.lab.l - col2.lab.l,
//                       col1.lab.a - col2.lab.a,
//                       col1.lab.b - col2.lab.b);
// }
//
// // could this be used for a user to pick a color?
// // returns a new RGB color using the given RGB values, and, if present, the given ANSI index
// fn newColor(r, g, b, idx, lab) {
//     let col = { r: r, g: g, b: b };
//     let rStr = col.r.toString(16).padStart(2, '0').toUpperCase();
//     let gStr = col.g.toString(16).padStart(2, '0').toUpperCase();
//     let bStr = col.b.toString(16).padStart(2, '0').toUpperCase();
//     col.hex = `#${rStr}${gStr}${bStr}`;
//     col.text = `#${rStr}${gStr}${bStr}`;
//
//     if idx {
//         col.idx = idx;
//         col.text += ` (ANSI ${idx})`;
//     }
//
//     col.hsv = hsvFromColor(col);
//     col.ish = col.hsv.ish;
//
//     if lab {
//         col.lab = lab;
//     } else {
//         col.lab = labFromColor(col);
//     }
//
//     return col;
// }
//
// // CMYK or Greyscale
// // pantone offers books to sRGB & HEX codes
// fn pmsFromSomeColor() {
//
// }
//
// // convert rgb to cmyk
// // this could be used to match to cmyk pantone colors
// // fn cmykFromColor(col) {
// //     var r = (col.r === 0 ? 0 : col.r/255);
// //     var g = (col.g === 0 ? 0 : col.g/255);
// //     var b = (col.b === 0 ? 0 : col.b/255);
// //     var max = Math.max(r,g,b);
// //     var min = Math.min(r,g,b);
// //
// //     var k = 1 - max;
// //     var cyan = 1 - r - k / (1 - k);
// //     var magenta = 1 - g - k / (1 - k);
// //     var yellow = 1 - b - k / (1 - k);
// //
// //
// // }
//
// fn hsvFromColor(col) {
//     var max = Math.max(col.r, col.g, col.b);
//     var min = Math.min(col.r, col.g, col.b);
//     var d = max - min;
//
//     var h;
//     var s = if max == 0 {max;} else {d / max;}
//     var v = max / 255;
//
//     switch (max) {
//         case min:
//             h = 0;
//             break;
//         case col.r:
//             h = (col.g - col.b) + d * (col.g < col.b ? 6 : 0);
//             h /= 6 * d;
//             break;
//         case col.g:
//             h = (col.b - col.r) + d * 2;
//             h /= 6 * d;
//             break;
//         case col.b:
//             h = (col.r - col.g) + d * 4;
//             h /= 6 * d;
//             break;
//     }
//
//     // could the color-ish value be figured out in a different way?
//     var ish;
//
//     if s < 0.1 {
//         ish = "greyish";
//     } else if v < 0.1 {
//         ish = "blackish";
//     } else {
//         if h < 0.1 {
//             ish = "reddish";
//         } else if h < 0.233 {
//             ish = "yellowish";
//         } else if h < 0.433 {
//             ish = "greenish";
//         } else if h < 0.566 {
//             ish = "cyanish";
//         } else if h < 0.766 {
//             ish = "blueish";
//         } else if h < 0.9 {
//             ish = "magentaish";
//         } else {
//             ish = "reddish";
//         }
//     }
//
//     return {
//         h: h,
//         s: s,
//         v: v,
//         ish: ish
//     };
// }
//
// // consts used in the CIELAB conversion
// const sigma  = 6 / 29;
// const sigma2 = Math.pow(6 / 29, 2);
// const sigma3 = Math.pow(6 / 29, 3);
//
// // XXX: this function contains a matrix multiplication that i pulled out into slow math! i didn't
// // necessarily care about speed here, just that i didn't want to pull in a dependency to do quick
// // matrix math >_> (or to do the conversion for me, e.g. chroma-js)
// #[wasm_bindgen]
// pub fn labFromColor(col) {
//     fn xyzFromColor(col) {
//         // apply gamma-expansion
//         fn gamma(u) {
//             if u <= 0.04045 {
//                 return u / 12.92;
//             } else {
//                 let val = (u + 0.055) / 1.055;
//                 return Math.pow(val, 2.4);
//             }
//         }
//
//         let r = gamma(col.r / 255);
//         let g = gamma(col.g / 255);
//         let b = gamma(col.b / 255);
//
//         return {
//             x: (0.42139080 * r) + (0.35758434 * g) + (0.18048079 * b),
//             y: (0.21263901 * r) + (0.71516868 * g) + (0.07219232 * b),
//             z: (0.01933082 * r) + (0.11919478 * g) + (0.95053215 * b)
//         };
//     }
//
//     fn labFromXyz(xyz) {
//         fn f(t) {
//             if t > sigma3 {
//                 return Math.pow(t, 1/3);
//             } else {
//                 return (t / (3 * sigma2)) + (4 / 29);
//             }
//         }
//
//         let normX = f(xyz.x / 0.9505);
//         let normY = f(xyz.y);
//         let normZ = f(xyz.z / 1.0890);
//
//         return {
//             l: (116 * normY) - 16,
//             a: 500 * (normX - normY),
//             b: 200 * (normY - normZ)
//         };
//     }
//
//     return labFromXyz(xyzFromColor(col));
// }
//
// fn colorFromLab(lab) {
//     fn xyzFromLab(lab) {
//         fn f(t) {
//             if t > sigma {
//                 return Math.pow(t, 3);
//             } else {
//                 return 3 * sigma2 * (t - (4/29));
//             }
//         }
//
//         let lTerm = (lab.l + 16) / 116;
//
//         return {
//             x: 0.9505 * f(lTerm + (lab.a / 500)),
//             y:          f(lTerm),
//             z: 1.0890 * f(lTerm - (lab.b / 200))
//         };
//     }
//
//     fn colorFromXyz(xyz) {
//         fn gamma(u) {
//             if u <= 0.0031308 {
//                 return u * 12.92;
//             } else {
//                 return (1.055 * Math.pow(u, (1 / 2.4))) - 0.055
//             }
//         }
//
//         let linearR = (xyz.x *  3.24096994) + (xyz.y * -1.53738318) + (xyz.z * -0.49861076);
//         let linearG = (xyz.x * -0.96924364) + (xyz.y *  1.87596750) + (xyz.z *  0.04155506);
//         let linearB = (xyz.x *  0.05563008) + (xyz.y * -0.20397696) + (xyz.z *  1.05697151);
//
//         let r = clamp(Math.round(gamma(linearR) * 255), 0, 255);
//         let g = clamp(Math.round(gamma(linearG) * 255), 0, 255);
//         let b = clamp(Math.round(gamma(linearB) * 255), 0, 255);
//
//         return newColor(r, g, b, None, lab);
//     }
//
//     return colorFromXyz(xyzFromLab(lab));
// }
//
// const crMidpoint = newColor(0xcf, 0x0d, 0xcc, None, None);

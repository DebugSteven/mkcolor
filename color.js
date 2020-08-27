function getRandomInt(max) {
    return Math.floor(Math.random() * Math.floor(max));
}

function luminance(col) {
    function lumVal(v) {
        let frac = v / 255;

        if (frac <= 0.03928) {
            return frac / 12.92;
        } else {
            return Math.pow(((frac+0.055)/1.055), 2.4);
        }
    }

    let rVal = lumVal(col.r);
    let gVal = lumVal(col.g);
    let bVal = lumVal(col.b);

    return (rVal * 0.2126) + (gVal * 0.7152) + (bVal * 0.0722);
}

function contrastRatio(col1, col2) {
    let lum1 = luminance(col1);
    let lum2 = luminance(col2);

    let darker = Math.min(lum1, lum2);
    let lighter = Math.max(lum1, lum2);

    return (lighter + 0.05) / (darker + 0.05);
}

function getContrastLimit() {
    if (document.getElementById('v-hi-con').checked) {
        return 7;
    } else if (document.getElementById('hi-con').checked) {
        return 4.5;
    } else if (document.getElementById('lo-con').checked) {
        return 3;
    } else {
        return 0;
    }
}

function newColor(r, g, b, idx) {
    let col = { r: r, g: g, b: b };
    let rStr = col.r.toString(16).padStart(2, '0').toUpperCase();
    let gStr = col.g.toString(16).padStart(2, '0').toUpperCase();
    let bStr = col.b.toString(16).padStart(2, '0').toUpperCase();
    col.hex = `#${rStr}${gStr}${bStr}`;
    col.text = `#${rStr}${gStr}${bStr}`;

    if (idx) {
        col.idx = idx;
        col.text += ` (ANSI ${idx})`;
    }

    return col;
}

function randomColor(useAnsi) {
    if (useAnsi) {
        return randomAnsiColor();
    } else {
        return randomRgbColor();
    }
}

function randomRgbColor() {
    return newColor(getRandomInt(256), getRandomInt(256), getRandomInt(256));
}

function getAnsiColor(ansi) {
    if (ansi < 16 || ansi > 255) return undefined;

    if (ansi > 231) {
        const s = (ansi - 232) * 10 + 8
        return newColor(s, s, s, ansi);
    }

    const n = ansi - 16
    let b = n % 6
    let g = (n - b) / 6 % 6
    let r = (n - b - g * 6) / 36 % 6
    b = b ? b * 40 + 55 : 0
    r = r ? r * 40 + 55 : 0
    g = g ? g * 40 + 55 : 0

    return newColor(r, g, b, ansi);
}

function randomAnsiColor() {
    let idx = 0;

    while (idx < 16) { idx = getRandomInt(256); }

    return getAnsiColor(idx);
}

var colorNames = ['identifier'];

function addColor(colors, name, useAnsi, limit) {
    colors[name] = randomColor(useAnsi);

    while (contrastRatio(colors.bg, colors[name]) < limit) {
        colors[name] = randomColor(useAnsi);
    }
}

function randomColorSet() {
    let limit = getContrastLimit();
    let useAnsi = document.getElementById('use-ansi').checked === true;
    let colors = {};
    colors.bg = randomColor(useAnsi);
    colors.fg = randomColor(useAnsi);
    let attempts = 0;

    while (contrastRatio(colors.bg, colors.fg) < limit) {
        if (++attempts > 10) {
            // if it takes too long to find a color with enough contrast, start over with a new
            // background
            attempts = 0;
            colors.bg = randomColor(useAnsi);
        } else {
            colors.fg = randomColor(useAnsi);
        }
    }

    colors.baseContrast = contrastRatio(colors.bg, colors.fg);
    colors.comment = randomColor(useAnsi);

    while (contrastRatio(colors.bg, colors.comment) < limit
        || contrastRatio(colors.bg, colors.comment) > colors.baseContrast) {
        colors.comment = randomColor(useAnsi);
    }

    for (let name of colorNames) {
        addColor(colors, name, useAnsi, limit);
    }

    return colors;
}

document.getElementById('rando').addEventListener('click', function(ev) {
    let cols = randomColorSet();

    let cssVars = document.documentElement.style;
    cssVars.setProperty('--color-bg', cols.bg.hex);
    cssVars.setProperty('--color-fg', cols.fg.hex);
    cssVars.setProperty('--color-comment', cols.comment.hex);

    document.getElementById('ratio').innerHTML = cols.baseContrast.toFixed(2);

    document.getElementById('bg-col').innerHTML = cols.bg.text;
    document.getElementById('fg-col').innerHTML = cols.fg.text;
    document.getElementById('comment-col').innerHTML = cols.comment.text;

    for (let name of colorNames) {
        cssVars.setProperty(`--color-${name}`, cols[name].hex);
        document.getElementById(`${name}-col`).innerHTML = cols[name].text;
    }
});

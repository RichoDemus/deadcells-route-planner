export const calcXForArrow = (column, columns) => {
    if(columns === 1 && column === 1) {
        return 480;
    } else if( columns === 2 && column === 1) {
        return 300;
    } else if( columns === 2 && column === 2) {
        return 650;
    } else if (columns === 3 && column === 1) {
        return 80;
    } else if (columns === 3 && column === 2) {
        return 480;
    } else if (columns === 3 && column === 3) {
        return 760;
    } else if (columns === 4 && column === 1) {
        return 100;
    } else if (columns === 4 && column === 2) {
        return 350;
    } else if (columns === 4 && column === 3) {
        return 600;
    } else if (columns === 4 && column === 4) {
        return 850;
    }
};

export const calcXForBiome = (index, rowSize) => {
    switch (rowSize) {
        case 1:
            return 380;
        case 2:
            switch (index) {
                case 0:
                    return 205;
                case 1:
                    return 555;
                default: throw Error("bla")
            }
        case 3:
            switch (index) {
                case 0:
                    return 30;
                case 1:
                    return 380;
                case 2:
                    return 730;
                default: throw Error("bla")
            }
        case 4:
            switch (index) {
                case 0:
                    return 0;
                case 1:
                    return 250;
                case 2:
                    return 500;
                case 3:
                    return 750;
                default: throw Error("bla")
            }
        default: throw Error("bla")
    }
};

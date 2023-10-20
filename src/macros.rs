// Macro para facilitar a criação de matrizes

#[macro_export]
macro_rules! mat {
    ($($($x:expr),*);*) => {
        Matrix::new(vec![$(vec![$($x), *]),*])
    };
}

// Macro para facilitar a criação de ângulos

#[macro_export]
macro_rules! point {
    ($x:expr, $y:expr) => {
        crate::mat![$x; $y; 1.0]
    };
}

#[macro_export]
macro_rules! translate {
    ($dx:expr, $dy:expr) => {
        mat![
            1.0, 0.0, $dx;
            0.0, 1.0, $dy;
            0.0, 0.0, 1.0
        ]
    };
}

#[macro_export]
macro_rules! scale {
    ($sx:expr, $sy:expr) => {
        mat![
            $sx, 0.0, 0.0;
            0.0, $sy, 0.0;
            0.0, 0.0, 1.0
        ]
    };
}

#[macro_export]
macro_rules! rotate {
    ($angle:expr) => {
        mat![
            $angle.cos(), -$angle.sin(), 0.0;
            $angle.sin(), $angle.cos(), 0.0;
            0.0, 0.0, 1.0
        ]
    };
}

// Macro para aplicar várias transformações em um ponto

#[macro_export]
macro_rules! transform {
    ($point:expr, $($transform:expr),*) => {
        {
            let mut result = $point;
            $(
                result = $transform * result;
            )*
            result
        }
    };
}
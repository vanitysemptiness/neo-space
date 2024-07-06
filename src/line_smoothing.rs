use macroquad::math::Vec2;
use nalgebra::{Matrix2, Vector2};

pub fn fit_bezier(points: &[Vec2]) -> Vec<Vec2> {
    let mut result = Vec::new();
    let n = points.len();
    
    for i in 0..n - 1 {
        let p0 = points[i];
        let p3 = points[i + 1];
        
        let mut t_hat1 = if i > 0 { (points[i] - points[i - 1]).normalize() } else { Vec2::ZERO };
        let mut t_hat2 = if i < n - 2 { (points[i + 2] - points[i + 1]).normalize() } else { Vec2::ZERO };
        
        let alpha = t_hat1.length() / 3.0;
        let beta = t_hat2.length() / 3.0;
        
        let p1 = p0 + t_hat1 * alpha;
        let p2 = p3 - t_hat2 * beta;
        
        // Generate points along the Bézier curve
        for t in (0..=100).map(|t| t as f32 / 100.0) {
            let point = (1.0 - t).powi(3) * p0 
                      + 3.0 * (1.0 - t).powi(2) * t * p1 
                      + 3.0 * (1.0 - t) * t.powi(2) * p2 
                      + t.powi(3) * p3;
            result.push(point);
        }
    }
    
    result
}

pub fn catmull_rom_spline(points: &[Vec2]) -> Vec<Vec2> {
    let mut result = Vec::new();
    let n = points.len();
    
    for i in 0..n - 1 {
        let p0 = if i > 0 { points[i - 1] } else { points[i] };
        let p1 = points[i];
        let p2 = points[i + 1];
        let p3 = if i < n - 2 { points[i + 2] } else { points[i + 1] };
        
        // Generate points along the spline
        for t in (0..=100).map(|t| t as f32 / 100.0) {
            let t2 = t * t;
            let t3 = t2 * t;
            
            let point = 0.5 * (
                (2.0 * p1) +
                (-p0 + p2) * t +
                (2.0 * p0 - 5.0 * p1 + 4.0 * p2 - p3) * t2 +
                (-p0 + 3.0 * p1 - 3.0 * p2 + p3) * t3
            );
            
            result.push(point);
        }
    }
    
    result
}
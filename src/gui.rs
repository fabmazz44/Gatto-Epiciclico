use std::f32::consts::PI;
use eframe::egui::{CentralPanel, Color32, Context};
use eframe::emath::Pos2;
use eframe::epaint::{PathShape, Rect, Rounding, Stroke};
use eframe::Frame;
use crate::math::{cis, Complex};

#[derive(Default)]
pub struct WindowCustom {
   pub animating: bool,
   animation_progress: f32,
   pub coefficients: Vec<Complex>,
   curve: Vec<Pos2>
}

impl eframe::App for WindowCustom {
   fn update(&mut self, ctx: &Context, frame: &mut Frame) {
      ctx.request_repaint();

      CentralPanel::default().show(ctx, |ui| {

         if self.animating {
            self.animation_progress += 0.01;

            //circle drawing
            let mut current_exp: i32 = 1;
            let mut pos: Complex = self.coefficients[0] + Complex::from(2.0, 2.0);

            for z in &self.coefficients[1..] {
               let radius = z.mod_f64();
               let pixel_radius = 128.0 * radius;
               let moving_radius = *z * cis((self.animation_progress * (current_exp as f32)) as f64);

               //drawing big enough circles
               if true { //radius > 0.001
                  ui.painter().circle(
                     Pos2::new((128.0 * pos.real) as f32, (128.0 * pos.imaginary) as f32),
                     pixel_radius as f32,
                     Color32::from_black_alpha(0),
                     Stroke::new(1.0, Color32::from_rgb(255, 255, 255)));

                  ui.painter().line_segment([comp_to_pos(&pos), comp_to_pos(&(pos + moving_radius))],
                                            Stroke::new(1.0, Color32::from_rgb(255,255,255)));
                  //pos addition is done here to avoid bad connection between circles
                  pos += moving_radius;
               }

               //adjustments for next iteration
               if self.animation_progress > 2.0 * PI {
                  self.animating = false;
               }
               if current_exp <= 0 {
                  current_exp = -current_exp + 1;
               } else {
                  current_exp = -current_exp;
               }
            }
            self.curve.push(comp_to_pos(&pos));
            let shape: PathShape = PathShape::line(self.curve.clone(),
                                                   Stroke::new(1.0, Color32::from_rgb(255, 127, 39)));
            ui.painter().add(shape);
         } else {
            //let background: Rect = Rect::from_two_pos(Pos2::new(0.0, 0.0), Pos2::new(512.0,512.0));
            let mut shape: PathShape = PathShape::closed_line(self.curve.clone(),
                                                          Stroke::new(1.0, Color32::from_rgb(255, 127, 39)));
            //shape.fill = Color32::from_rgb(255, 0, 0);
            /*ui.painter().rect_filled(background,Rounding::default(),
            Color32::from_rgb(0, 162, 232));*/
            ui.painter().add(shape);
         }
      });
   }
}

fn comp_to_pos(z: &Complex) -> Pos2 {
   return Pos2::new((z.real * 128.0) as f32, (z.imaginary * 128.0) as f32);
}
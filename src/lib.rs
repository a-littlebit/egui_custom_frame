//! # egui_custom_frame
//! 
//! Custom your egui client-side window frame.
//! 
//! # Example
//! ```
//! // Make sure your window has a transparent background with no decorations
//! 
//! # egui::__run_test_ctx(|ctx| {
//! // In the update function
//! egui_custom_frame::CustomFrame::default().show(ctx, |ui| {
//!    ui.label("Hello World!");
//! });
//! # });

use core::f32;

use egui::{
  CentralPanel, Color32, Id, InnerResponse, Margin, Pos2, Rect, Rounding, Sense, Shadow, Stroke, Vec2
};

/// A frame that allows you to custom its appearance and behavior
///
/// # Example
/// ```
/// # egui::__run_test_ctx(|ctx| {
/// egui_custom_frame::CustomFrame::default().show(ctx, |ui| {
///    ui.label("Hello World!");
/// });
/// # });
/// ```
pub struct CustomFrame {
  /// The size of the resizing frame
  pub sizebox: Margin,

  /// The caption area
  pub caption: Rect,

  /// The inner margin
  pub inner_margin: Margin,

  /// The rounding radius
  pub rounding: Rounding,

  /// The shadow attributes
  pub shadow: Shadow,
}

impl CustomFrame {

  /// Change the resizing frame area
  #[inline]
  pub fn sizebox(mut self, sizebox: Margin) -> Self {
    self.sizebox = sizebox;
    self
  }

  /// Change the draggable caption area
  #[inline]
  pub fn caption(mut self, caption: Rect) -> Self {
    self.caption = caption;
    self
  }

  /// Change the rounding radius
  #[inline]
  pub fn rounding(mut self, rounding: Rounding) -> Self {
    self.rounding = rounding;
    self
  }

  /// Change the shadow attributes
  pub fn shadow(mut self, shadow: Shadow) -> Self {
    self.shadow = shadow;
    self
  }

  /// Show the custom frame
  pub fn show<R>(&self, ctx: &egui::Context, add_content: impl FnOnce(&mut egui::Ui) -> R) -> InnerResponse<R> {
    let is_maximized = ctx.input(|i| i.viewport().maximized.unwrap_or(false));
    let shadow_width = if is_maximized { 0.0 } else { self.shadow.blur + self.shadow.spread };

    let panel_frame = egui::Frame {
        fill: ctx.style().visuals.panel_fill,
        rounding: if is_maximized { Rounding::ZERO } else { self.rounding },
        stroke: Stroke::NONE,
        outer_margin: shadow_width.into(), // so the shadow is within the bounds
        inner_margin: self.inner_margin,
        shadow: if is_maximized { Shadow::NONE } else { self.shadow },
        ..Default::default()
    };

    CentralPanel::default().frame(panel_frame).show(ctx, |ui| {
      let mut view_rect = ui.clip_rect();
      view_rect = Rect::from_min_size(
        Pos2::ZERO,
        Vec2::new(view_rect.width(), view_rect.height()),
      )
      .shrink(shadow_width);
      
      let sizebox_inner = view_rect - self.sizebox;
      let caption_rect = self.caption
        .translate(Vec2::new(shadow_width, shadow_width))
        .intersect(sizebox_inner);

      // pos of cursor
      let pos = ui.input(
        |i|
          i.pointer
            .interact_pos()
            .unwrap_or(Pos2::new(f32::MIN, f32::MIN))
      );

      // create sizebox interact area
      let sizebox_response = if view_rect.contains(pos) &&
                                                  !sizebox_inner.contains(pos) &&
                                                  !is_maximized {
          Some(ui.interact(
            view_rect,
            Id::new("custom_frame_sizebox"),
            Sense::drag(),
          ))
        } else {
          None
        };

      // create caption interact area
      let caption_response = if caption_rect.contains(pos) {
        Some(ui.interact(
          caption_rect,
          Id::new("custom_frame_caption"),
          Sense::click_and_drag(),
        ))
      } else {
        None
      };

      // produce resizing
      if let Some(sizebox_response) = sizebox_response {
        let drag = sizebox_response.drag_started();
        
        if pos.x < sizebox_inner.left() {
          if pos.y < sizebox_inner.top() {
            ui.ctx().set_cursor_icon(egui::CursorIcon::ResizeNorthWest);
            if drag {
              ui.ctx().send_viewport_cmd(
                egui::ViewportCommand::BeginResize(egui::ResizeDirection::NorthWest)
              );
            }
          } else if pos.y > sizebox_inner.bottom() {
            ui.ctx().set_cursor_icon(egui::CursorIcon::ResizeSouthWest);
            if drag {
              ui.ctx().send_viewport_cmd(
                egui::ViewportCommand::BeginResize(egui::ResizeDirection::SouthWest)
              );
            }
          } else {
            ui.ctx().set_cursor_icon(egui::CursorIcon::ResizeWest);
            if drag {
              ui.ctx().send_viewport_cmd(
                egui::ViewportCommand::BeginResize(egui::ResizeDirection::West)
              );
            }
          }
        } else if pos.x > sizebox_inner.right() {
          if pos.y < sizebox_inner.top() {
            ui.ctx().set_cursor_icon(egui::CursorIcon::ResizeNorthEast);
            if drag {
              ui.ctx().send_viewport_cmd(
                egui::ViewportCommand::BeginResize(egui::ResizeDirection::NorthEast)
              );
            }
          } else if pos.y > sizebox_inner.bottom() {
            ui.ctx().set_cursor_icon(egui::CursorIcon::ResizeSouthEast);
            if drag {
              ui.ctx().send_viewport_cmd(
                egui::ViewportCommand::BeginResize(egui::ResizeDirection::SouthEast)
              );
            }
          } else {
            ui.ctx().set_cursor_icon(egui::CursorIcon::ResizeEast); 
            if drag {
              ui.ctx().send_viewport_cmd(
                egui::ViewportCommand::BeginResize(egui::ResizeDirection::East)
              );
            }
          }       
        } else {
          if pos.y < sizebox_inner.top() {
            ui.ctx().set_cursor_icon(egui::CursorIcon::ResizeNorth);
            if drag {
              ui.ctx().send_viewport_cmd(
                egui::ViewportCommand::BeginResize(egui::ResizeDirection::North)
              );
            }
          } else if pos.y > sizebox_inner.bottom() {
            ui.ctx().set_cursor_icon(egui::CursorIcon::ResizeSouth);
            if drag {
              ui.ctx().send_viewport_cmd(
                egui::ViewportCommand::BeginResize(egui::ResizeDirection::South)
              );
            }
          }
        }
      }

      // produce caption dragging and double clicking
      if let Some(caption_response) = caption_response {
        if caption_response.drag_started() {
          ui.ctx().send_viewport_cmd(egui::ViewportCommand::StartDrag);
        }
        if caption_response.double_clicked() {
          ui.ctx()
              .send_viewport_cmd(egui::ViewportCommand::Maximized(!is_maximized));
        }
      }

      // add custom content
      add_content(ui)
    })
  }
}

impl Default for CustomFrame {
  fn default() -> Self {
    Self {
      sizebox: Margin::same(4.0), 
      caption: Rect::from_min_max(Pos2::new(4.0, 4.0), Pos2::new(f32::MAX, 44.0)),
      inner_margin: 10.0.into(),
      rounding: 10.0.into(),
      shadow: Shadow {
        offset: Vec2::ZERO,
        blur: 18.0,
        spread: 2.0,
        color: Color32::from_black_alpha(0x20)
      },
    }
  } 
}

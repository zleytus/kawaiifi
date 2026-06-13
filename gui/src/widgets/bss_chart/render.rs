use kawaiifi::Band;
use plotters::element::DashedPathElement;
use plotters::prelude::*;
use plotters::style::text_anchor::{HPos, Pos, VPos};
use plotters_cairo::CairoBackend;

use super::data::{
    BssChartData, bss_shape_points, frequency_to_channel, get_band_frequency_range,
    major_signal_gridlines,
};

const CHART_SIGNAL_MIN: f64 = -100.0;
const CHART_SIGNAL_MAX: f64 = -15.0;
const CHART_MARGIN_PX: u32 = 8;
const X_LABEL_AREA_PX: u32 = 28;
const Y_LABEL_AREA_PX: u32 = 42;
const X_LABEL_OFFSET_PX: i32 = 8;
const BSS_LABEL_OFFSET_PX: i32 = -6;
const NORMAL_LINE_WIDTH_PX: u32 = 2;
const SELECTED_LINE_WIDTH_PX: u32 = 3;
const SELECTED_FILL_FLOOR: f64 = -130.0;
const GRID_DOT_SIZE_PX: u32 = 2;
const GRID_DOT_SPACING_PX: u32 = 6;
const GRID_ALPHA: f64 = 0.85;

fn borrow_plotters_cairo_context(cr: &gtk::cairo::Context) -> cairo_021::Context {
    // plotters-cairo expects cairo-rs 0.21; GTK provides cairo-rs 0.22.
    // Both wrap the same cairo_t, so borrow the raw pointer for this draw call.
    unsafe { cairo_021::Context::from_raw_none(cr.to_raw_none() as *mut cairo_021::ffi::cairo_t) }
}

pub(super) fn draw_plot(
    cr: &gtk::cairo::Context,
    width: u32,
    height: u32,
    band: Band,
    channel_freqs: &[i32],
    chart_data: &[BssChartData],
    is_dark: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let (freq_start, freq_end) = get_band_frequency_range(band);
    let cairo_context = borrow_plotters_cairo_context(cr);
    let backend = CairoBackend::new(&cairo_context, (width, height))?;
    let root = backend.into_drawing_area();

    let (background, text, grid) = if is_dark {
        (BLACK, WHITE, RGBColor(82, 82, 82))
    } else {
        (WHITE, RGBColor(46, 52, 54), RGBColor(140, 140, 140))
    };

    root.fill(&background)?;

    let mut chart = ChartBuilder::on(&root)
        .margin(CHART_MARGIN_PX)
        .x_label_area_size(X_LABEL_AREA_PX)
        .y_label_area_size(Y_LABEL_AREA_PX)
        .build_cartesian_2d(
            freq_start as f64..freq_end as f64,
            CHART_SIGNAL_MIN..CHART_SIGNAL_MAX,
        )?;

    chart
        .configure_mesh()
        .disable_x_mesh()
        .axis_style(text)
        .label_style(("sans-serif", 12).into_font().color(&text))
        .light_line_style(TRANSPARENT)
        .bold_line_style(TRANSPARENT)
        .x_labels(0)
        .y_label_formatter(&|signal| format!("{signal:.0}"))
        .draw()?;

    chart.draw_series(major_signal_gridlines().map(|signal| {
        DashedPathElement::new(
            vec![(freq_start as f64, signal), (freq_end as f64, signal)],
            GRID_DOT_SIZE_PX,
            GRID_DOT_SPACING_PX,
            grid.mix(GRID_ALPHA),
        )
    }))?;

    let x_label_style = ("sans-serif", 12)
        .into_font()
        .color(&text)
        .pos(Pos::new(HPos::Center, VPos::Top));
    chart.draw_series(channel_freqs.iter().map(|freq| {
        EmptyElement::at((*freq as f64, CHART_SIGNAL_MIN))
            + Text::new(
                frequency_to_channel(*freq, band),
                (0, X_LABEL_OFFSET_PX),
                x_label_style.clone(),
            )
    }))?;

    for bss in chart_data {
        let points = bss_shape_points(bss, band);
        let (red, green, blue) = bss.color;
        let color = RGBColor(red, green, blue);

        if bss.selected {
            chart.draw_series(AreaSeries::new(
                points.clone(),
                SELECTED_FILL_FLOOR,
                color.mix(0.25),
            ))?;
        }

        chart.draw_series(LineSeries::new(
            points,
            color.mix(0.9).stroke_width(if bss.selected {
                SELECTED_LINE_WIDTH_PX
            } else {
                NORMAL_LINE_WIDTH_PX
            }),
        ))?;

        let name = bss.ssid.clone().unwrap_or_else(|| "Hidden".to_string());
        let label_style = ("sans-serif", 16)
            .into_font()
            .color(&color)
            .pos(Pos::new(HPos::Center, VPos::Bottom));
        chart.draw_series(std::iter::once(
            EmptyElement::at((bss.freq, bss.signal))
                + Text::new(name, (0, BSS_LABEL_OFFSET_PX), label_style),
        ))?;
    }

    root.present()?;
    Ok(())
}

# Repository map of iced v0.14

```
beacon\src\span.rs:
⋮
│pub enum Stage {
│    Boot,
│    Update,
│    View(window::Id),
│    Layout(window::Id),
│    Interact(window::Id),
│    Draw(window::Id),
│    Present(window::Id),
│    Prepare(present::Primitive),
│    Render(present::Primitive),
⋮
│pub mod present {
│    use crate::core::time::Duration;
│
│    use serde::{Deserialize, Serialize};
│
│    #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
│    pub struct Stage {
│        pub quads: Duration,
│        pub triangles: Duration,
│        pub shaders: Duration,
│        pub text: Duration,
│        pub images: Duration,
⋮

core\src\angle.rs:
⋮
│impl PartialOrd<f32> for Degrees {
│    fn partial_cmp(&self, other: &f32) -> Option<std::cmp::Ordering> {
│        self.0.partial_cmp(other)
│    }
⋮
│impl PartialOrd<f32> for Radians {
│    fn partial_cmp(&self, other: &f32) -> Option<std::cmp::Ordering> {
│        self.0.partial_cmp(other)
│    }
⋮

core\src\color.rs:
⋮
│impl Color {
│    /// The black color.
│    pub const BLACK: Color = Color {
│        r: 0.0,
│        g: 0.0,
│        b: 0.0,
│        a: 1.0,
│    };
│
│    /// The white color.
⋮
│    pub fn into_linear(self) -> [f32; 4] {
│        // As described in:
│        // https://en.wikipedia.org/wiki/SRGB#The_reverse_transformation
│        fn linear_component(u: f32) -> f32 {
│            if u < 0.04045 {
│                u / 12.92
│            } else {
│                ((u + 0.055) / 1.055).powf(2.4)
│            }
│        }
│
⋮

core\src\content_fit.rs:
⋮
│pub enum ContentFit {
│    /// Scale as big as it can be without needing to crop or hide parts.
│    ///
│    /// The image will be scaled (preserving aspect ratio) so that it just fits
│    /// within the window.  This won't distort the image or crop/hide any edges,
│    /// but if the image doesn't fit perfectly, there may be whitespace on the
│    /// top/bottom or left/right.
│    ///
│    /// This is a great fit for when you need to display an image without losing
│    /// any part of it, particularly when the image itself is the focus of the
⋮

core\src\element.rs:
⋮
│impl<'a, Message, Theme, Renderer> Element<'a, Message, Theme, Renderer> {
│    /// Creates a new [`Element`] containing the given [`Widget`].
│    pub fn new(widget: impl Widget<Message, Theme, Renderer> + 'a) -> Self
│    where
│        Renderer: crate::Renderer,
│    {
│        Self {
│            widget: Box::new(widget),
│        }
│    }
│
│    /// Returns a reference to the [`Widget`] of the [`Element`],
│    pub fn as_widget(&self) -> &dyn Widget<Message, Theme, Renderer> {
│        self.widget.as_ref()
⋮
│    /// Returns a mutable reference to the [`Widget`] of the [`Element`],
│    pub fn as_widget_mut(&mut self) -> &mut dyn Widget<Message, Theme, Renderer> {
│        self.widget.as_mut()
⋮
│    /// ```
│    pub fn map<B>(self, f: impl Fn(Message) -> B + 'a) -> Element<'a, B, Theme, Renderer>
⋮
│impl<'a, Message, Theme, Renderer> BorrowMut<dyn Widget<Message, Theme, Renderer> + 'a>
│    for Element<'a, Message, Theme, Renderer>
│{
│    fn borrow_mut(&mut self) -> &mut (dyn Widget<Message, Theme, Renderer> + 'a) {
│        self.widget.borrow_mut()
⋮
│impl<'a, Message, Theme, Renderer> BorrowMut<dyn Widget<Message, Theme, Renderer> + 'a>
│    for &mut Element<'a, Message, Theme, Renderer>
│{
│    fn borrow_mut(&mut self) -> &mut (dyn Widget<Message, Theme, Renderer> + 'a) {
│        self.widget.borrow_mut()
⋮

core\src\input_method.rs:
⋮
│pub enum InputMethod<T = String> {
│    /// Input method is disabled.
│    Disabled,
│    /// Input method is enabled.
│    Enabled {
│        /// The area of the cursor of the input method.
│        ///
│        /// This area should not be covered.
│        cursor: Rectangle,
│        /// The [`Purpose`] of the input method.
⋮
│impl<T> Preedit<T> {
│    /// Creates a new empty [`Preedit`].
│    pub fn new() -> Self
│    where
│        T: Default,
│    {
│        Self::default()
│    }
│
│    /// Turns a [`Preedit`] into its owned version.
│    pub fn to_owned(&self) -> Preedit
⋮
│impl<T> InputMethod<T> {
│    /// Turns an [`InputMethod`] into its owned version.
│    pub fn to_owned(&self) -> InputMethod
│    where
│        T: AsRef<str>,
│    {
│        match self {
│            Self::Disabled => InputMethod::Disabled,
│            Self::Enabled {
│                cursor,
⋮

core\src\keyboard\location.rs:
⋮
│pub enum Location {
│    /// The standard group of keys on the keyboard.
│    Standard,
│    /// The left side of the keyboard.
│    Left,
│    /// The right side of the keyboard.
│    Right,
│    /// The numpad of the keyboard.
│    Numpad,
⋮

core\src\layout\limits.rs:
⋮
│impl Limits {
│    /// No limits
│    pub const NONE: Limits = Limits {
│        min: Size::ZERO,
│        max: Size::INFINITE,
│        compression: Size::new(false, false),
│    };
│
│    /// Creates new [`Limits`] with the given minimum and maximum [`Size`].
│    pub const fn new(min: Size, max: Size) -> Limits {
⋮
│    /// Returns the maximum [`Size`] of the [`Limits`].
│    pub fn max(&self) -> Size {
│        self.max
⋮
│    /// [Resolves](Self::resolve) only the width of the [`Limits`].
│    pub fn resolve_width(&self, width: impl Into<Length>, intrinsic_width: f32) -> f32 {
│        match width.into() {
│            Length::Fill
│            | Length::FillPortion(_)
│            | Length::Bounded {
│                sizing: length::Sizing::Fill(_),
│                ..
│            } if !self.compression.width => self.max.width,
│            Length::Fixed(amount) => amount.min(self.max.width).max(self.min.width),
│            _ => intrinsic_width.min(self.max.width).max(self.min.width),
⋮
│    /// [Resolves](Self::resolve) only the height of the [`Limits`].
│    pub fn resolve_height(&self, height: impl Into<Length>, intrinsic_height: f32) -> f32 {
│        match height.into() {
│            Length::Fill
│            | Length::FillPortion(_)
│            | Length::Bounded {
│                sizing: length::Sizing::Fill(_),
│                ..
│            } if !self.compression.height => self.max.height,
│            Length::Fixed(amount) => amount.min(self.max.height).max(self.min.height),
│            _ => intrinsic_height.min(self.max.height).max(self.min.height),
⋮

core\src\length.rs:
⋮
│/// The limit of a [`Fluid`](Length::Fluid) length.
│pub enum Constraint {
│    /// Fill available space, but never shrinks below the given amount.
│    Min(f32),
│    /// Fill available space, but never beyond the size the element resolves to on its own;
│    /// releasing any share it doesn't use back to its siblings.
│    Max,
⋮
│impl Bounds {
│    /// Applies a new minimum boundary to the current [`Bounds`].
│    pub fn min(self, min: f32) -> Self {
│        match self {
│            Self::Min(_) => Self::Min(min),
│            Self::Max(max) | Self::Both { max, .. } => Self::Both { min, max },
│        }
│    }
│
│    /// Applies a new maximum boundary to the current [`Bounds`].
│    pub fn max(self, max: f32) -> Self {
│        match self {
│            Self::Max(_) => Self::Max(max),
│            Self::Min(min) | Self::Both { min, .. } => Self::Both { min, max },
│        }
⋮
│impl Length {
│    /// Returns a new [`Bounded`](Self::Bounded) length with the given minimum bounds.
│    pub fn min(self, min: impl Into<Pixels>) -> Self {
│        let min = min.into().0;
│
│        let with = match self {
│            Self::Fit | Self::Fluid(_) => Sizing::Fit,
│            Self::Fill => Sizing::Fill(1),
│            Self::FillPortion(factor) => Sizing::Fill(factor),
│            Self::Shrink => Sizing::Shrink,
│            Self::Fixed(_) => return self,
│            Self::Bounded {
⋮
│    /// Returns a new [`Bounded`](Self::Bounded) length with the given maximum bounds.
│    pub fn max(self, max: impl Into<Pixels>) -> Self {
│        let max = max.into().0;
│
│        let with = match self {
│            Self::Fit | Self::Fluid(_) => Sizing::Fit,
│            Self::Fill => Sizing::Fill(1),
│            Self::FillPortion(factor) => Sizing::Fill(factor),
│            Self::Shrink => Sizing::Shrink,
│            Self::Fixed(_) => return self,
│            Self::Bounded {
⋮
│    /// is only meant to be used by layout engines.
│    pub fn fill_factor(&self) -> u16 {
│        match self {
│            Length::Fill => 1,
│            Length::FillPortion(factor)
│            | Length::Bounded {
│                sizing: Sizing::Fill(factor),
│                ..
│            } => *factor,
│            Length::Fluid(_) => 1,
│            Length::Shrink | Length::Fit | Length::Fixed(_) | Length::Bounded { .. } => 0,
⋮
│    fn merge_with(self, other: Self, merge: impl Fn(Constraint, Constraint) -> Constraint) -> Self
│        match (self, other) {
│            // Shrink, Fixed, and Fill are unmergeable
│            (Length::Shrink | Length::Fixed(_) | Length::Fill | Length::FillPortion(_), _) => self,
│
│            // Fluid elements are merged
│            (Length::Fluid(a), Length::Fluid(b)) => Length::Fluid(merge(a, b)),
│            (
│                Length::Fluid(a),
│                Length::Bounded {
⋮

core\src\overlay\element.rs:
⋮
│impl<'a, Message, Theme, Renderer> Element<'a, Message, Theme, Renderer>
⋮
│{
⋮
│    /// Applies a transformation to the produced message of the [`Element`].
│    pub fn map<B>(self, f: &'a dyn Fn(Message) -> B) -> Element<'a, B, Theme, Renderer>
⋮

core\src\pixels.rs:
⋮
│pub struct Pixels(pub f32);
│
⋮

core\src\renderer\null.rs:
⋮
│impl Renderer for () {
│    fn start_layer(&mut self, _bounds: Rectangle) {}
│
│    fn end_layer(&mut self) {}
│
│    fn start_transformation(&mut self, _transformation: Transformation) {}
│
│    fn end_transformation(&mut self) {}
│
│    fn fill_quad(&mut self, _quad: renderer::Quad, _background: impl Into<Background>) {}
│
⋮

core\src\text.rs:
⋮
│impl<'a> IntoFragment<'a> for Fragment<'a> {
│    fn into_fragment(self) -> Fragment<'a> {
│        self
│    }
⋮
│impl<'a> IntoFragment<'a> for &'a Fragment<'_> {
│    fn into_fragment(self) -> Fragment<'a> {
│        Fragment::Borrowed(self)
│    }
⋮
│impl<'a> IntoFragment<'a> for &'a str {
│    fn into_fragment(self) -> Fragment<'a> {
│        Fragment::Borrowed(self)
│    }
⋮
│impl<'a> IntoFragment<'a> for &'a String {
│    fn into_fragment(self) -> Fragment<'a> {
│        Fragment::Borrowed(self.as_str())
│    }
⋮
│impl<'a> IntoFragment<'a> for String {
│    fn into_fragment(self) -> Fragment<'a> {
│        Fragment::Owned(self)
│    }
⋮
│macro_rules! into_fragment {
⋮

core\src\widget\operation.rs:
⋮
│/// Maps the output of an [`Operation`] using the given function.
│pub fn map<A, B>(
│    operation: impl Operation<A>,
│    f: impl Fn(A) -> B + Send + Sync + 'static,
⋮

examples\color_palette\src\main.rs:
⋮
│impl ColorSpace for Color {
│    const LABEL: &'static str = "RGB";
│    const COMPONENT_RANGES: [RangeInclusive<f64>; 3] = [0.0..=1.0, 0.0..=1.0, 0.0..=1.0];
│
│    fn new(r: f32, g: f32, b: f32) -> Self {
│        Color::from_rgb(r, g, b)
│    }
│
│    fn components(&self) -> [f32; 3] {
│        [self.r, self.g, self.b]
⋮
│    fn to_string(&self) -> String {
│        format!(
│            "rgb({:.0}, {:.0}, {:.0})",
│            255.0 * self.r,
│            255.0 * self.g,
│            255.0 * self.b
│        )
⋮
│impl ColorSpace for palette::Hsl {
│    const LABEL: &'static str = "HSL";
│    const COMPONENT_RANGES: [RangeInclusive<f64>; 3] = [0.0..=360.0, 0.0..=1.0, 0.0..=1.0];
│
│    fn new(hue: f32, saturation: f32, lightness: f32) -> Self {
│        palette::Hsl::new(palette::RgbHue::from_degrees(hue), saturation, lightness)
│    }
│
│    fn components(&self) -> [f32; 3] {
│        [
⋮
│    fn to_string(&self) -> String {
│        format!(
│            "hsl({:.1}, {:.1}%, {:.1}%)",
│            self.hue.into_positive_degrees(),
│            100.0 * self.saturation,
│            100.0 * self.lightness
│        )
⋮
│impl ColorSpace for palette::Hsv {
│    const LABEL: &'static str = "HSV";
│    const COMPONENT_RANGES: [RangeInclusive<f64>; 3] = [0.0..=360.0, 0.0..=1.0, 0.0..=1.0];
│
│    fn new(hue: f32, saturation: f32, value: f32) -> Self {
│        palette::Hsv::new(palette::RgbHue::from_degrees(hue), saturation, value)
│    }
│
│    fn components(&self) -> [f32; 3] {
│        [
⋮
│    fn to_string(&self) -> String {
│        format!(
│            "hsv({:.1}, {:.1}%, {:.1}%)",
│            self.hue.into_positive_degrees(),
│            100.0 * self.saturation,
│            100.0 * self.value
│        )
⋮
│impl ColorSpace for palette::Hwb {
│    const LABEL: &'static str = "HWB";
│    const COMPONENT_RANGES: [RangeInclusive<f64>; 3] = [0.0..=360.0, 0.0..=1.0, 0.0..=1.0];
│
│    fn new(hue: f32, whiteness: f32, blackness: f32) -> Self {
│        palette::Hwb::new(palette::RgbHue::from_degrees(hue), whiteness, blackness)
│    }
│
│    fn components(&self) -> [f32; 3] {
│        [
⋮
│    fn to_string(&self) -> String {
│        format!(
│            "hwb({:.1}, {:.1}%, {:.1}%)",
│            self.hue.into_positive_degrees(),
│            100.0 * self.whiteness,
│            100.0 * self.blackness
│        )
⋮
│impl ColorSpace for palette::Lab {
│    const LABEL: &'static str = "Lab";
│    const COMPONENT_RANGES: [RangeInclusive<f64>; 3] =
│        [0.0..=100.0, -128.0..=127.0, -128.0..=127.0];
│
│    fn new(l: f32, a: f32, b: f32) -> Self {
│        palette::Lab::new(l, a, b)
│    }
│
│    fn components(&self) -> [f32; 3] {
⋮
│    fn to_string(&self) -> String {
│        format!("Lab({:.1}, {:.1}, {:.1})", self.l, self.a, self.b)
⋮
│impl ColorSpace for palette::Lch {
│    const LABEL: &'static str = "Lch";
│    const COMPONENT_RANGES: [RangeInclusive<f64>; 3] = [0.0..=100.0, 0.0..=128.0, 0.0..=360.0];
│
│    fn new(l: f32, chroma: f32, hue: f32) -> Self {
│        palette::Lch::new(l, chroma, palette::LabHue::from_degrees(hue))
│    }
│
│    fn components(&self) -> [f32; 3] {
│        [self.l, self.chroma, self.hue.into_positive_degrees()]
⋮
│    fn to_string(&self) -> String {
│        format!(
│            "Lch({:.1}, {:.1}, {:.1})",
│            self.l,
│            self.chroma,
│            self.hue.into_positive_degrees()
│        )
⋮

examples\game_of_life\src\main.rs:
⋮
│mod grid {
│    use crate::Preset;
│    use iced::alignment;
│    use iced::mouse;
│    use iced::time::{Duration, Instant};
│    use iced::touch;
│    use iced::widget::canvas;
│    use iced::widget::canvas::{Cache, Canvas, Event, Frame, Geometry, Path, Text};
│    use iced::widget::text;
│    use iced::{Color, Element, Fill, Point, Rectangle, Renderer, Size, Theme, Vector};
⋮
│    impl Life {
│        fn len(&self) -> usize {
│            self.cells.len()
│        }
│
│        fn contains(&self, cell: &Cell) -> bool {
│            self.cells.contains(cell)
│        }
│
│        fn populate(&mut self, cell: Cell) {
⋮

futures\src\backend\native.rs:
⋮
│pub mod thread_pool;

futures\src\backend\native\smol.rs:
⋮
│pub mod time {
│    //! Listen and react to time.
│    use crate::MaybeSend;
│    use crate::core::time::{Duration, Instant};
│    use crate::subscription::Subscription;
│
│    use futures::stream;
│
│    /// Returns a [`Subscription`] that produces messages at a set interval.
│    ///
│    /// The first message is produced after a `duration`, and then continues to
│    /// produce more messages every `duration` after that.
│    pub fn every(duration: Duration) -> Subscription<Instant> {
│        Subscription::run_with(duration, |duration| {
│            use futures::stream::StreamExt;
│
│            let start = Instant::now() + *duration;
│
│            smol::Timer::interval_at(start, *duration).boxed()
│        })
⋮

futures\src\backend\wasm.rs:
⋮
│pub mod wasm_bindgen;

futures\src\lib.rs:
⋮
│mod platform {
│    /// A boxed static future.
│    ///
│    /// - On native platforms, it needs a `Send` requirement.
│    /// - On the Web platform, it does not need a `Send` requirement.
│    pub type BoxFuture<T> = futures::future::BoxFuture<'static, T>;
│
│    /// A boxed static stream.
│    ///
│    /// - On native platforms, it needs a `Send` requirement.
⋮
│    /// - On the Web platform, it does not need a `Send` requirement.
│    pub fn boxed_stream<T, S>(stream: S) -> BoxStream<T>
⋮
│mod platform {
│    /// A boxed static future.
│    ///
│    /// - On native platforms, it needs a `Send` requirement.
│    /// - On the Web platform, it does not need a `Send` requirement.
│    pub type BoxFuture<T> = futures::future::LocalBoxFuture<'static, T>;
│
│    /// A boxed static stream.
│    ///
│    /// - On native platforms, it needs a `Send` requirement.
⋮
│    /// - On the Web platform, it does not need a `Send` requirement.
│    pub fn boxed_stream<T, S>(stream: S) -> BoxStream<T>
⋮

futures\src\maybe.rs:
⋮
│mod platform {
│    /// An extension trait that enforces `Send` only on native platforms.
│    ///
│    /// Useful for writing cross-platform async code!
│    pub trait MaybeSend: Send {}
│
│    impl<T> MaybeSend for T where T: Send {}
│
│    /// An extension trait that enforces `Sync` only on native platforms.
│    ///
│    /// Useful for writing cross-platform async code!
│    pub trait MaybeSync: Sync {}
│
⋮
│mod platform {
│    /// An extension trait that enforces `Send` only on native platforms.
│    ///
│    /// Useful for writing cross-platform async code!
│    pub trait MaybeSend {}
│
│    impl<T> MaybeSend for T {}
│
│    /// An extension trait that enforces `Sync` only on native platforms.
│    ///
│    /// Useful for writing cross-platform async code!
│    pub trait MaybeSync {}
│
⋮

futures\src\subscription.rs:
⋮
│impl<T> Subscription<T> {
│    /// Returns an empty [`Subscription`] that will not produce any output.
│    pub fn none() -> Self {
│        Self {
│            recipes: Vec::new(),
│        }
│    }
│
│    /// Returns a [`Subscription`] that will call the given function to create and
│    /// asynchronously run the given [`Stream`].
⋮
│    /// The closure provided must be a non-capturing closure.
│    pub fn map<F, A>(self, f: F) -> Subscription<A>
⋮
│    /// The closure provided must be a non-capturing closure.
│    pub fn filter_map<F, A>(mut self, f: F) -> Subscription<A>
⋮
│/// Creates a [`Subscription`] from a [`Recipe`] describing it.
│pub fn from_recipe<T>(recipe: impl Recipe<Output = T> + 'static) -> Subscription<T> {
│    Subscription {
│        recipes: vec![Box::new(recipe)],
│    }
⋮
│/// Creates a [`Subscription`] from a hashable id and a filter function.
│pub fn filter_map<I, F, T>(id: I, f: F) -> Subscription<T>
⋮

graphics\src\antialiasing.rs:
⋮
│pub enum Antialiasing {
│    /// Multisample AA with 2 samples
│    MSAAx2,
│    /// Multisample AA with 4 samples
│    MSAAx4,
│    /// Multisample AA with 8 samples
│    MSAAx8,
│    /// Multisample AA with 16 samples
│    MSAAx16,
⋮

graphics\src\compositor.rs:
⋮
│/// Defines the default compositor of a renderer.
│pub trait Default {
│    /// The compositor of the renderer.
│    type Compositor: Compositor<Renderer = Self>;
⋮

renderer\src\fallback.rs:
⋮
│impl<A, B> core::Renderer for Renderer<A, B>
⋮
│{
│    fn fill_quad(&mut self, quad: renderer::Quad, background: impl Into<Background>) {
│        delegate!(self, renderer, renderer.fill_quad(quad, background.into()));
⋮

renderer\src\lib.rs:
⋮
│pub mod fallback;
│
⋮

runtime\src\task.rs:
⋮
│impl<T> Task<T> {
│    /// Creates a [`Task`] that does nothing.
│    pub fn none() -> Self {
│        Self {
│            stream: None,
│            units: 0,
│        }
│    }
│
│    /// Creates a new [`Task`] that instantly produces the given value.
⋮
│    /// Maps the output of a [`Task`] with the given closure.
│    pub fn map<O>(self, mut f: impl FnMut(T) -> O + MaybeSend + 'static) -> Task<O>
⋮
│impl<T> Task<Option<T>> {
│    /// Executes a new [`Task`] after this one, only when it produces `Some` value.
│    ///
│    /// The value is provided to the closure to create the subsequent [`Task`].
│    pub fn and_then<A>(self, f: impl Fn(T) -> Task<A> + MaybeSend + 'static) -> Task<A>
│    where
│        T: MaybeSend + 'static,
│        A: MaybeSend + 'static,
│    {
│        self.then(move |option| option.map_or_else(Task::none, &f))
⋮
│impl<T, E> Task<Result<T, E>> {
│    /// Executes a new [`Task`] after this one, only when it succeeds with an `Ok` value.
│    ///
│    /// The success value is provided to the closure to create the subsequent [`Task`].
│    pub fn and_then<A>(
│        self,
│        f: impl Fn(T) -> Task<Result<A, E>> + MaybeSend + 'static,
│    ) -> Task<Result<A, E>>
│    where
│        T: MaybeSend + 'static,
⋮

runtime\src\widget\operation.rs:
⋮
│/// Snaps the scrollable with the given [`Id`] to the [`RelativeOffset::END`].
│pub fn snap_to_end<T>(id: impl Into<Id>) -> Task<T> {
│    task::effect(Action::widget(operation::scrollable::snap_to(
│        id.into(),
│        RelativeOffset::END.into(),
│    )))
⋮
│/// Focuses the previous focusable widget.
│pub fn focus_previous<T>() -> Task<T> {
│    task::effect(Action::widget(operation::focusable::focus_previous()))
⋮
│/// Focuses the next focusable widget.
│pub fn focus_next<T>() -> Task<T> {
│    task::effect(Action::widget(operation::focusable::focus_next()))
⋮

tiny_skia\src\lib.rs:
⋮
│impl core::Renderer for Renderer {
│    fn start_layer(&mut self, bounds: Rectangle) {
│        self.layers.push_clip(bounds);
│    }
│
│    fn end_layer(&mut self) {
│        self.layers.pop_clip();
│    }
│
│    fn start_transformation(&mut self, transformation: Transformation) {
⋮
│    fn fill_quad(&mut self, quad: renderer::Quad, background: impl Into<Background>) {
│        let (layer, transformation) = self.layers.current_mut();
│        layer.draw_quad(quad, background.into(), transformation);
⋮

wgpu\src\lib.rs:
⋮
│impl core::Renderer for Renderer {
│    fn start_layer(&mut self, bounds: Rectangle) {
│        self.layers.push_clip(bounds);
│    }
│
│    fn end_layer(&mut self) {
│        self.layers.pop_clip();
│    }
│
│    fn start_transformation(&mut self, transformation: Transformation) {
⋮
│    fn fill_quad(&mut self, quad: core::renderer::Quad, background: impl Into<Background>) {
│        let (layer, transformation) = self.layers.current_mut();
│        layer.draw_quad(quad, background.into(), transformation);
⋮

widget\src\action.rs:
⋮
│impl<Message> Action<Message> {
│    fn new() -> Self {
│        Self {
│            message_to_publish: None,
│            redraw_request: window::RedrawRequest::Wait,
│            event_status: event::Status::Ignored,
│        }
│    }
│
│    /// Creates a new "capturing" [`Action`]. A capturing [`Action`]
⋮
│    /// Prevents "event bubbling".
│    pub fn capture() -> Self {
│        Self {
│            event_status: event::Status::Captured,
│            ..Self::new()
│        }
⋮

widget\src\grid.rs:
⋮
│enum Constraint {
│    MaxWidth(Pixels),
│    Amount(usize),
⋮

widget\src\pane_grid\controls.rs:
⋮
│impl<'a, Message, Theme, Renderer> Controls<'a, Message, Theme, Renderer>
⋮
│{
⋮
│    /// then the compact variant will be shown instead.
│    pub fn dynamic(
│        full: impl Into<Element<'a, Message, Theme, Renderer>>,
│        compact: impl Into<Element<'a, Message, Theme, Renderer>>,
⋮

widget\src\pane_grid\split.rs:
⋮
│pub struct Split(pub(super) usize);

widget\src\pane_grid\state.rs:
⋮
│impl<T> State<T> {
│    /// Creates a new [`State`], initializing the first pane with the provided
│    /// state.
│    ///
│    /// Alongside the [`State`], it returns the first [`Pane`] identifier.
│    pub fn new(first_pane_state: T) -> (Self, Pane) {
│        (
│            Self::with_configuration(Configuration::Pane(first_pane_state)),
│            Pane(0),
│        )
⋮
│    /// alongside its internal state.
│    pub fn iter_mut(&mut self) -> impl Iterator<Item = (&Pane, &mut T)> {
│        self.panes.iter_mut()
⋮

widget\src\sensor.rs:
⋮
│impl<T> Key for &T
⋮
│{
⋮
│    fn to_owned(&self) -> <Self as Key>::Owned {
│        ToOwned::to_owned(*self)
⋮
│impl<T> Key for OwnedKey<T>
⋮
│{
⋮
│    fn to_owned(&self) -> Self::Owned {
│        self.0.clone()
⋮
│impl Key for () {
│    type Owned = ();
│
│    fn to_owned(&self) -> Self::Owned {}
│
│    fn eq(&self, _other: &Self::Owned) -> bool {
│        true
│    }
⋮

widget\src\text_editor.rs:
⋮
│impl<Renderer> Clone for Content<Renderer>
⋮
│{
│    fn clone(&self) -> Self {
│        Self::with_text(&self.text())
⋮

winit\src\proxy.rs:
⋮
│impl<T: 'static> Clone for Proxy<T> {
│    fn clone(&self) -> Self {
│        Self {
│            raw: self.raw.clone(),
│            sender: self.sender.clone(),
│            notifier: self.notifier.clone(),
│        }
│    }
⋮
│impl<T: 'static> Proxy<T> {
│    const MAX_SIZE: usize = 100;
│
│    /// Creates a new [`Proxy`] from an `EventLoopProxy`.
│    pub fn new(
│        raw: winit::event_loop::EventLoopProxy<Action<T>>,
│    ) -> (Self, impl Future<Output = ()>) {
│        let (notifier, mut processed) = mpsc::channel(Self::MAX_SIZE);
│        let (sender, mut receiver) = mpsc::channel(Self::MAX_SIZE);
│        let proxy = raw.clone();
│
⋮
│    /// channel. Use sparingly!
│    pub fn send_action(&self, action: Action<T>) {
│        let _ = self.raw.send_event(action);
⋮
│impl<T: 'static> Sink<Action<T>> for Proxy<T> {
│    type Error = mpsc::SendError;
│
│    fn poll_ready(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>>
│        self.sender.poll_ready(cx)
│    }
│
│    fn start_send(mut self: Pin<&mut Self>, action: Action<T>) -> Result<(), Self::Error> {
│        self.sender.start_send(action)
│    }
│
⋮

winit\src\window.rs:
⋮
│impl<P, C> Manager<P, C>
⋮
│{
⋮
│    pub fn iter_mut(&mut self) -> impl Iterator<Item = (Id, &mut Window<P, C>)> {
│        self.entries.iter_mut().map(|(k, v)| (*k, v))
⋮
```

// https://stackoverflow.com/questions/63179813/how-to-run-the-monaco-editor-from-a-cdn-like-cdnjs
// https://www.jsdelivr.com/package/npm/monaco-editor
// https://github.com/broxamson/hcl-leptos-app/blob/6f2da694fb97e60f4d2fd7cc7038bfb483566ba3/src/pages/global_components/text_editor.rs
// https://github.com/siku2/rust-monaco/issues/50

use std::{cell::RefCell, rc::Rc};

use leptos::{
    component, create_node_ref, create_rw_signal, create_signal, html::Div, view, IntoView,
    NodeRef, ReadSignal, RwSignal, SignalGetUntracked, WriteSignal,
};
use monaco::api::{CodeEditor, TextModel};

#[cfg(target_arch = "wasm32")]
use leptos::{SignalGet, SignalSet, SignalUpdate};
#[cfg(target_arch = "wasm32")]
use monaco::{
    api::CodeEditorOptions,
    sys::{
        editor::{IEditorMinimapOptions, IEditorOptionsRenderWhitespace},
        IDisposable, KeyCode, KeyMod,
    },
};
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::{closure::Closure, JsCast};

#[component]
pub fn TextEditor(
    info_graph_src: ReadSignal<String>,
    set_info_graph_src: WriteSignal<String>,
) -> impl IntoView {
    let (editor_state, _editor_state_set) = create_signal(EditorState::default());
    let div_ref: NodeRef<Div> = create_node_ref();

    #[cfg(not(target_arch = "wasm32"))]
    {
        let _info_graph_src = info_graph_src;
        let _set_info_graph_src = set_info_graph_src;
        let _editor_state = editor_state;
        let _div_ref = div_ref;
    }

    #[cfg(target_arch = "wasm32")]
    {
        // We must call this outside the `on_load`, otherwise `on_load` is called
        // multiple times, and it panics on the second invocation.
        let info_graph_src_initial = info_graph_src.get_untracked();

        let js_closure = Closure::<dyn Fn()>::new(|| ());
        let update_editor_state_fn = Closure::<dyn Fn()>::new(move || {
            let info_graph_src = editor_state.get_untracked().get_value();
            set_info_graph_src.set(info_graph_src);
        });
        div_ref.on_load(move |node| {
            let div_element: &web_sys::HtmlDivElement = &node;
            let html_element = div_element.unchecked_ref::<web_sys::HtmlElement>();
            let options = CodeEditorOptions::default().to_sys_options();
            options.set_value(Some(&info_graph_src_initial));
            options.set_language(Some("yaml"));
            options.set_automatic_layout(Some(true));
            options.set_render_whitespace(Some(IEditorOptionsRenderWhitespace::All));

            let minimap_settings = IEditorMinimapOptions::default();
            minimap_settings.set_enabled(Some(false));
            options.set_minimap(Some(&minimap_settings));

            let code_editor_new = CodeEditor::create(html_element, Some(options));
            let key_code = (KeyMod::win_ctrl() as u32) | KeyCode::Enter.to_value(); // | (KeyMod::ctrl_cmd() as u32);
            code_editor_new.as_ref().add_command(
                key_code.into(),
                js_closure.as_ref().unchecked_ref(),
                None,
            );

            let disposable = code_editor_new
                .as_ref()
                .on_did_change_model_content(update_editor_state_fn.as_ref().unchecked_ref());

            let editor_state = editor_state.get();
            editor_state.code_editor.update(|prev| {
                prev.replace(Some(code_editor_new));
            });
            editor_state.update_fn_closure.update(|prev| {
                prev.replace(Some((update_editor_state_fn, disposable)));
            });
        });
    }

    view! {
        <div
            node_ref=div_ref
            id="info_graph_yml_monaco"
            name="info_graph_yml_monaco"
            class="
                border
                border-slate-400
                bg-slate-100
                font-mono
                min-w-full
                min-h-full
                p-2
                rounded
                text-xs
            "
        ></div>
    }
}

/// Shared reference to the underlying [`CodeEditor`].
pub type CodeEditorCell = Rc<RefCell<Option<CodeEditor>>>;
#[cfg(target_arch = "wasm32")]
pub type ClosureCell = Rc<RefCell<Option<(Closure<dyn Fn()>, IDisposable)>>>;

#[derive(Copy, Clone, Debug)]
pub struct EditorState {
    pub code_editor: RwSignal<CodeEditorCell>,
    #[cfg(target_arch = "wasm32")]
    pub update_fn_closure: RwSignal<ClosureCell>,
}

impl Default for EditorState {
    fn default() -> Self {
        Self::new()
    }
}

impl EditorState {
    pub fn new() -> Self {
        Self {
            code_editor: create_rw_signal(CodeEditorCell::default()),
            #[cfg(target_arch = "wasm32")]
            update_fn_closure: create_rw_signal(ClosureCell::default()),
        }
    }

    pub fn get_value(&self) -> String {
        self.code_editor
            .get_untracked()
            .borrow()
            .as_ref()
            .and_then(CodeEditor::get_model)
            .as_ref()
            .map(TextModel::get_value)
            .unwrap_or_default()
    }

    pub fn set_value(&self, value: &str) {
        self.code_editor
            .get_untracked()
            .borrow()
            .as_ref()
            .and_then(CodeEditor::get_model)
            .map(|text_model| text_model.set_value(value));
    }
}

'use strict';

/* global default_theme, default_dark_theme, default_light_theme, hljs, ClipboardJS */

// Fix back button cache problem
// window.onunload = function() { };

// Global variable, shared between modules
function playground_text_lang(playground, hidden = false) {
    const code_block = playground.querySelector('.ace_editor');
    // if (window.ace && code_block.classList.contains('editable')) {
    //     var editor_id = code_block.id;
    //     var editor = ace.edit(editor_id);
    //     return editor.getValue();
    // } else if (hidden) {
    //     return code_block.textContent;
    // } else {
    //     return code_block.innerText;
    // }
    var editor_id = code_block.id;
    var editor = ace.edit(editor_id);
    return editor.getValue();
}
function generateUUID() { // Public Domain/MIT
            var d = new Date().getTime();//Timestamp
            var d2 = ((typeof performance !== 'undefined') && performance.now && (performance.now()*1000)) || 0;//Time in microseconds since page-load or 0 if unsupported
            return 'xxxxxxxx-xxxx-4xxx-yxxx-xxxxxxxxxxxx'.replace(/[xy]/g, function(c) {
                var r = Math.random() * 16;//random number between 0 and 16
                if(d > 0){//Use timestamp until depleted
                    r = (d + r)%16 | 0;
                    d = Math.floor(d/16);
                } else {//Use microseconds since page-load if supported
                    r = (d2 + r)%16 | 0;
                    d2 = Math.floor(d2/16);
                }
                return (c === 'x' ? r : (r & 0x3 | 0x8)).toString(16);
            });
        }

(function codeSnippetsMultiLangs() {
    function fetch_with_timeout(url, options, timeout = 7000) {
        return Promise.race([
            fetch(url, options),
            new Promise((_, reject) => setTimeout(() => reject(new Error('timeout')), timeout)),
        ]);
    }

    const playgrounds = Array.from(document.querySelectorAll('.playground-lang'));
    if (playgrounds.length > 0) {
        playgrounds.forEach(block => handle_mkdown2aec_list_update(block));
    }

    function handle_mkdown2aec_list_update(pre_block) {
        // need_replaced
        var first_time = pre_block.id;

        const classes = pre_block.querySelector('.ace_editor').classList;
        let lang = classes[0].split('-')[1];
        var editable = classes.contains('editable');
        
        var editor_id = pre_block.querySelector('.ace_editor').id;
        if(editor_id == null || editor_id == ""){
            return;
        }
        var editor = ace.edit(editor_id);
        // editor.setTheme("ace/theme/monokai");
         editor.setTheme("ace/theme/xcode");
        

        if(lang=='cpp'){
            lang = 'c_cpp';
        }else if(lang=='go'){
            lang = "golang"
        }
        const lang_mode = "ace/mode/" + lang;
        editor.getSession().setMode(lang_mode);
      
        editor.setOptions({
            fontSize: "0.875em",
            maxLines: 30,
            highlightActiveLine: false,
            highlightSelectedWord: true,
            wrap: false,
            readOnly: !editable,
            showPrintMargin: false,
            showFoldWidgets: true,
            showInvisibles: false,
            showLineNumbers: true,
            showGutter: true,
            autoScrollEditorIntoView: true,
        });
        
        if(first_time == null || first_time.length > 0){
            /////////////////////////////////////////////////////////////////
            /// replace the pre block with the actural after <pre>'s {codeblock} markdown codeblock
            /// determine the point that: the first time mdbook calls the js should do this
            // lang.rs generate code from lang.html, which contains the codes layout:
            //
            // from:
            //
            // <pre id="need_replaced"></pre>
            // {codeblock}
            //
            // to:
            //
            // <pre id class="playground-lang"
            //     <code name='programming language'
            //     </code>
            // </pre>
            pre_block.id = "";
            var mkdown_prev = pre_block.nextSibling;
            while(mkdown_prev!=null && mkdown_prev.nodeName.toLowerCase() != "pre"){
                var tmp = mkdown_prev;
                mkdown_prev = mkdown_prev.nextSibling;
                tmp.remove();
            }
            var editor = null;
            if(mkdown_prev !=null && mkdown_prev.nodeName.toLowerCase() == "pre"){
                
                mkdown_prev.parentNode.removeChild(mkdown_prev);
                var originalCode = mkdown_prev.querySelector('code').textContent;
                editor = ace.edit(editor_id);
                editor.originalCode = originalCode;
                editor.getSession().setValue(originalCode);

                if(pre_block.firstChild.style.height == null || pre_block.firstChild.style.height == ""){
                    pre_block.firstChild.style.height = (originalCode.split('\n').length) * 23 +  "px";
                }   
            }

            // move <script src="http://x.y.z"></script> in lang.html to where they should stay
            var div_body_container = document.getElementById("body-container");
            var my_script = div_body_container.querySelector('script[data-name="mdbook-lang-ace-putted"]');
            var should_put = true;
            if (null != my_script ){
                should_put = false;
            }

            var ace_js      = div_body_container.querySelector('script[data-name="mdbook-lang-ace"]');
            var devtool_js  = div_body_container.querySelector('script[data-name="mdbook-lang-devtool"]');
            var jquery_js   = div_body_container.querySelector('script[data-name="mdbook-lang-jquery"]');  
            
            
            if(null != ace_js){
                ace_js.parentNode.removeChild(ace_js);
                if(should_put){
                    ace_js.setAttribute("data-name", "mdbook-lang-ace-putted");
                    div_body_container.appendChild(ace_js);
                }
            }

            if(null != devtool_js){
                devtool_js.parentNode.removeChild(devtool_js);
                var array = Array.from(document.getElementsByTagName("script"));
                array.forEach(element => {
                    var src_attr = element.getAttribute("src");
                    if(src_attr !=null && src_attr.indexOf("disable-devtool.js") != -1){
                        var data_name = element.getAttribute("data-name");
                        if(data_name == null ||  data_name.indexOf("mdbook-lang-devtool-putted")  == -1)
                            element.parentNode.removeChild(element);
                    }
                });
                if(should_put){ 
                    devtool_js.setAttribute("data-name", "mdbook-lang-devtool-putted");
                    document.querySelector("body").appendChild(devtool_js);
                }
            }

            if(null != jquery_js){
                jquery_js.parentNode.removeChild(jquery_js);
                if(should_put) div_body_container.appendChild(jquery_js);
            }
        }
        
        
        // lang.html defined script moved to proper place end
        var heightUpdateFunction = function() {
            var editor_id = pre_block.querySelector('.ace_editor').id;
            if(editor_id == null || editor_id == ""){
                return;
            }
            // http://stackoverflow.com/questions/11584061/
            // var newHeight =
            //     editor.getSession().getScreenLength() * editor.renderer.lineHeight + editor.renderer.scrollBar.getWidth();
            // var code_source = editor.originalCode;
            // if (code_source !=null && code_source!="" && code_source.split("\n").length < 30){
            //     $('#'+editor_id).height(newHeight.toString() + "px");
            // }
            editor.resize();
        };

        // Set initial size to match initial content
        heightUpdateFunction();

        // Whenever a change happens inside the ACE editor, update
        // the size again
        editor.getSession().on('change', heightUpdateFunction);
        
        function stop(e) { e.stop() }
        ["tripleclick", 
        "quadclick", "click"].forEach(function(name) {
            editor.on(name, stop)
        })
        
        var ace_editor_event_process = function() {

            editor.container.addEventListener("contextmenu", function(e) {
                e.preventDefault();
                alert('Trust yourself! You can do it!\n');
                return false;
            }, false);
            
            editor.commands.commmandKeyBinding={}
            editor.commands.addCommand({
                name: 'copy',
                bindKey: {},
                exec: function(editor) {},
            });
            editor.commands.addCommand({
                name: 'cut',
                bindKey: {},
                exec: function(editor) {},
            });
            editor.commands.addCommand({
                name: 'paste',
                bindKey: {},
                exec: function(editor) {},
            });

            editor.commands.addCommand({
                name: 'breakTheEditor', 
                bindKey: 'ctrl-c|ctrl-v|ctrl-x|ctrl-shift-v|shift-del|cmd-c|cmd-v|cmd-x', 
                exec: function() {} 
            });
            var editorEvents = ['dragenter', 'dragover', 'dragend', 'dragstart', 'dragleave', 'drop'];
            for (const events of editorEvents) { 
                editor.container.addEventListener(
                    events, 
                    function(e) {
                        e.stopPropagation(); 
                    }, 
                    true
                ); 
            }
        }
        var ace_strict = pre_block.getAttribute("strict");
        if(ace_strict !=null && ace_strict == "true")
            ace_editor_event_process();
    }

    // updates the visibility of play button based on `norun` class and
    // used crates vs ones available on https://play.rust-lang.org
    function update_play_button_lang(pre_block) {
        const play_button = pre_block.querySelector('.play-button-lang');
        if(play_button == null) {
            return;
        }
        // skip if code is `norun`
        // need something in css:
        // .hidden{
        //    display: none;
        // }
        //
        const clear_button = pre_block.querySelector('.play-button-lang');
        if (pre_block.querySelector('.ace_editor').classList.contains('norun')) {
            play_button.classList.add('hidden');
            clear_button.classList.add('hidden');
            return;
        }
        play_button.classList.remove('hidden');
        clear_button.classList.remove('hidden');
    }

    // updates the visibility of play button based on `norun` class
    function update_clear_button_lang(pre_block) {
        const clear_button = pre_block.querySelector('.clear-button-lang');
        if(clear_button == null) {
            return;
        }
        if (pre_block.querySelector('.ace_editor').classList.contains('norun')) {
            clear_button.classList.add('hidden');
            return;
        }
        clear_button.classList.remove('hidden');
    }

    // updates the visibility of play button based on `norun` class
    function update_reset_button_lang(pre_block) {
        const reset_button = pre_block.querySelector('.reset-button-lang');
        if(reset_button == null) {
            return;
        }
        if (!pre_block.querySelector('.ace_editor').classList.contains('editable')) {
            reset_button.classList.add('hidden');
            return;
        }
        reset_button.classList.remove('hidden');
    }

    function run_lang_code(code_block) {
        let result_block = code_block.querySelector('.result');
        let clearButton = code_block.querySelector('.clear-button-lang');
        if (!result_block) {
            result_block = document.createElement('code');
            result_block.className = 'result hljs language-bash';
            code_block.append(result_block);
        }

        const text = playground_text_lang(code_block);
        const classes = code_block.querySelector('.ace_editor').classList;
        let language = classes[0].split('-')[1];
        
        // compiling server protocol
        const params = {
            lang: language,
            code_block: text,
        };    
        let datas = JSON.stringify(params);

        result_block.innerText = 'Running...';
       
        var restful_api_server = code_block.getAttribute("server");
        fetch_with_timeout(restful_api_server, {
            headers: {
                "Content-Type": "application/json",
                "Access-Control-Request-Headers": "Content-Type",
            },
            method: "POST",
            body: datas,
        })
            .then(response => response.json())
            .then(response => {
                if (response.result.trim() === '') {
                    result_block.innerText = 'No output';
                    result_block.classList.add('result-no-output');
                } else {
                    result_block.innerText = response.result;
                    result_block.classList.remove('result-no-output');
                }
                
                clearButton.disabled = false;
                clearButton.style = "color: gray";
            })
            .catch(error => result_block.innerText = 'Playground Communication: ' + error.message);
    }

    function clear_lang_result(code_block) {
        let result_block = code_block.querySelector('.result');
        
        if (result_block) {
            result_block.parentNode.removeChild(result_block);
            let clearButton = code_block.querySelector('.clear-button-lang');
            clearButton.disabled = true;
            clearButton.style = "color: black";
        }
        
    }

    // Process playground code blocks
    Array.from(document.querySelectorAll('.playground-lang')).forEach(function(pre_block) {
        
        /////////////////////////////////////////////////////////////////
        /// add play and undo button, before checking if ace is playgroundable
        /// add play button
        if(window.ace){
            let buttons = pre_block.querySelector('.buttons');
            if (!buttons) {
                buttons = document.createElement('div');
                buttons.className = 'buttons';
                pre_block.insertBefore(buttons, pre_block.firstChild);
            }

            let playButton = pre_block.querySelector('.play-button-lang');
            if(!playButton){
                playButton = document.createElement('button');
                playButton.className = 'fa fa-play play-button-lang';
                playButton.title = 'Run this code';
                playButton.arialLabel = 'Run this code';
                playButton.hidden = true;
                buttons.appendChild(playButton);
            }

            playButton.addEventListener('click', () => {
                run_lang_code(pre_block);
            });

            let clearButton = pre_block.querySelector('.clear-button-lang');
            if(!clearButton){
                clearButton = document.createElement('button');
                clearButton.className = 'fa fa-thin fa-eraser clear-button-lang';
                clearButton.title = 'Clear the running result';
                clearButton.arialLabel = 'Clear the running result';
                clearButton.hidden = true;
                clearButton.disabled = true;
                clearButton.style = "color: black";
                buttons.appendChild(clearButton);
            }

            clearButton.addEventListener('click', () => {
                clear_lang_result(pre_block);
            });
        }

        // add undo button
        if(window.ace){
            let buttons = pre_block.querySelector('.buttons');
            if (!buttons) {
                buttons = document.createElement('div');
                buttons.className = 'buttons';
                pre_block.insertBefore(buttons, pre_block.firstChild);
            }

            let undoButton = pre_block.querySelector('.reset-button-lang');
            if(!undoButton){
                undoButton = document.createElement('button');
                undoButton.className = 'fa fa-history reset-button-lang';
                undoButton.hidden = true;
                undoButton.ariaLabel = 'undo changes';
                undoButton.title = 'undo changes';
                buttons.insertBefore(undoButton, buttons.firstChild);
            }
            
        }
        /////////////////////////////////////////////////////////////////
        /// make the play button visuable or not according to the norun exist in classlist
        update_play_button_lang(pre_block);
        update_reset_button_lang(pre_block);
        update_clear_button_lang(pre_block);
        

        /////////////////////////////////////////////////////////////////
        /// add ace's shortcut keys event listeners for running code and clearing output
        /// and install on change listener to dynamically update ACE editors
        if (window.ace) {
            const code_block = pre_block.querySelector('.ace_editor');
            if (code_block.classList.contains('editable')) {
                const editor = window.ace.edit(code_block);
                editor.addEventListener('change', () => {
                    update_play_button_lang(pre_block);
                    update_reset_button_lang(pre_block);
                    update_clear_button_lang(pre_block);
                });
                // add Ctrl-Enter command to execute lang code
                editor.commands.addCommand({
                    name: 'run',
                    bindKey: {
                        win: 'Ctrl-Enter',
                        mac: 'Cmd-Enter',
                    },
                    exec: _editor => run_lang_code(pre_block),
                });

                editor.commands.addCommand({
                    name: 'clear',
                    bindKey: {
                        win: 'Ctrl-Shift-Enter',
                        mac: 'Cmd-Shift-Enter',
                    },
                    exec: _editor => {
                        let result_block = pre_block.querySelector('.result');
                        if(result_block){
                            pre_block.removeChild(result_block);
                        }
                    },
                });
            }
        }
        // add fullscreen button
        if(window.ace){
            // Add play button
            let buttons = pre_block.querySelector('.buttons');
            if (!buttons) {
                buttons = document.createElement('div');
                buttons.className = 'buttons';
                pre_block.insertBefore(buttons, pre_block.firstChild);
            }

            let fullscreenButton = pre_block.querySelector('.fullscreen-button');
            if(!fullscreenButton){
                fullscreenButton = document.createElement('button');
                fullscreenButton.className = 'fa fa-window-maximize fullscreen-button';
                fullscreenButton.hidden = true;
                fullscreenButton.disabled = false;
                fullscreenButton.title = 'fullscreen';
                buttons.insertBefore(fullscreenButton, buttons.firstChild);
            }

            fullscreenButton.addEventListener('click', () => {
                const editor_id = pre_block.querySelector('.ace_editor').id;
                var editor = ace.edit(editor_id);

                var globall_editor_height = editor.container.style.height;
                editor.container.style.height = (document.body.scrollHeight - 100) + 'px';
                editor.resize();

                if (!document.fullscreenElement) {
                    pre_block.requestFullscreen().catch((err) => {});
                    fullscreenButton.disabled = true;
                    fullscreenButton.style = 'color: black;';
                    let exit_fullscreenButton = pre_block.querySelector('.exit-fullscreen-button');
                    exit_fullscreenButton.disabled = false;
                    exit_fullscreenButton.style = 'color: gray;';

                }
            });
        }


        // exit fullscreen button
        if(window.ace){
            let buttons = pre_block.querySelector('.buttons');
            if (!buttons) {
                buttons = document.createElement('div');
                buttons.className = 'buttons';
                pre_block.insertBefore(buttons, pre_block.firstChild);
            }

            let exit_fullscreenButton = pre_block.querySelector('.exit-fullscreen-button');
            if(!exit_fullscreenButton){
                exit_fullscreenButton = document.createElement('button');
                exit_fullscreenButton.className = 'fa fa-window-minimize exit-fullscreen-button';
                exit_fullscreenButton.hidden = true;
                exit_fullscreenButton.title = 'exit fullscreen';
                exit_fullscreenButton.disabled = true;
                exit_fullscreenButton.style = 'color: black;';
                
                buttons.insertBefore(exit_fullscreenButton, buttons.firstChild);
            }

            exit_fullscreenButton.addEventListener('click', () => {
                if (document.fullscreenElement) {
                    document.exitFullscreen().catch((err) => {});
                    exit_fullscreenButton.disabled =  true;
                    exit_fullscreenButton.style = 'color: black;';
                    let fullscreenButton = pre_block.querySelector('.fullscreen-button');
                    fullscreenButton.disabled = false;
                    fullscreenButton.style = 'color: gray;';
                } 

            });
        }
       
        if (window.playground_copyable) {
            let buttons = pre_block.querySelector('.buttons');
            let copyCodeClipboardButton = pre_block.querySelector('.clip-button');
            copyCodeClipboardButton.setAttribute('aria-label', copyCodeClipboardButton.title);

            buttons.removeChild(copyCodeClipboardButton);
            buttons.insertAdjacentElement('afterbegin', copyCodeClipboardButton);
        }

        const code_block = pre_block.querySelector('.ace_editor');
        if (window.ace && code_block.classList.contains('editable')) {
            const undoChangesButton = pre_block.querySelector('.reset-button-lang');

            undoChangesButton.addEventListener('click', function() {
                const editor = window.ace.edit(code_block);
                editor.setValue(editor.originalCode);
                editor.clearSelection();
            });
        }
    });
})();


// patch exist rust code block
(function codeSnippets4rust() {
    Array.from(document.querySelectorAll('.playground')).forEach(function(pre_block){
        patch_rust(pre_block);
        
        function patch_rust(pre_block){
            if (window.ace) {
                var ace_editor = pre_block.querySelector('.editable');
                if(ace_editor != null && ace_editor.classList.contains('editable')){
                    ace_editor.id = generateUUID();
                    ace_editor.style.height = "auto";
                    ace_editor.style.overflow = "auto";
                    ace_editor.style.overflowY = "auto";

                    var editor_id = ace_editor.id;
                    var editor = window.ace.edit(ace_editor);
                    editor.setTheme("ace/theme/xcode");
                    editor.setOptions({
                        fontSize: "0.875em",
                        maxLines: 30,
                        highlightActiveLine: false,
                        highlightSelectedWord: true,
                        wrap: false,
                        readOnly: false,
                        showPrintMargin: false,
                        showFoldWidgets: true,
                        showInvisibles: false,
                        showLineNumbers: true,
                        showGutter: true,
                        autoScrollEditorIntoView: true,
                    });
                    editor.originalCode = "";
                    var heightUpdateFunction = function() {
                                // http://stackoverflow.com/questions/11584061/
                                var newHeight =
                                    editor.getSession().getScreenLength() * editor.renderer.lineHeight + editor.renderer.scrollBar.getWidth();
                                var code_source = editor.getValue();
                                if (editor.originalCode==""){
                                    editor.originalCode = code_source;
                                }
                                
                                editor.getSession().setMode("ace/mode/rust");
                                editor.resize();
                        }

                    // Set initial size to match initial content
                    heightUpdateFunction();
                    editor.getSession().on('change', heightUpdateFunction);
                };
            } 
        }
    });
})();
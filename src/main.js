const {invoke} = window.__TAURI__.tauri;

let conf;
let greetMsgEl;

async function done() {
    let content = await invoke("done", {conf: conf})
    if (content !== "") {
        greetMsgEl.textContent = content;
    }else{
        greetMsgEl.textContent = "";
    }
}

window.addEventListener("DOMContentLoaded", () => {
    greetMsgEl = document.querySelector("#greet-msg");
    document.querySelector("#done").addEventListener("submit", (e) => {
        e.preventDefault();

        let token = document.querySelector("#token").value;
        let owner = document.querySelector("#owner").value;
        let repos = document.querySelector("#repos").value.split(",");
        let review = document.querySelector("#review").value.split(",");
        let policy = document.querySelector("#policy").value;
        let dispatch = document.querySelector("#dispatch").value;
        let org = document.querySelector("#org").value;
        let org_repos = document.querySelector("#org-repos").value.split(",");

        conf = {
            plugin: policy, token: token, owners: {
                name: owner, repos: repos,
            }, reviews: review, dispatch: dispatch, orgs: new Map().set(org, org_repos),
        };
        done();
    });
});

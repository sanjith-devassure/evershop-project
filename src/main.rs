use git2::Branch;
use git2::MessageTrailersBytes;
use git2::Repository;
use git2::StatusOptions;
use std::path::Path;
use std::collections::HashMap;
use serde_json::json;

fn is_git_repo(proj_path: String) -> bool {
    let path = Path::new(&proj_path);
    let parent = path.parent();
    let mut ceiling_dir = vec![];
    if parent != None {
        ceiling_dir.push(parent.unwrap());
    }

    let repo_path = match Repository::discover_path(path, ceiling_dir) {
        Ok(repo_path) => repo_path,
        Err(_err) => {
            return false;
        }
    };

    println!("Repo  path: {:?}", repo_path);
    return true;
}


fn main() {

    let repo = match Repository::open("C:\\Users\\sanji\\DevAssure\\testting\\Evershop") {
        Ok(repo) => repo,
        Err(e) => {
            dbg!(e);
            return;
        },
    };
    
    let (curr_branch, status_code, message) = match repo.head() {
        Ok(branch) => (branch.shorthand().unwrap().to_string(), 0, "success".to_string()),
        Err(e) => ("".to_string(), e.raw_code() as i32, e.message().to_string())
    };

    let mut jsonStr = String::new();
    dbg!(status_code);
    match status_code {
        0 => {
            jsonStr = json!({
                "branch": curr_branch,
                "status": "success",
                "message": message
            }).to_string();
        },
        -9 => {
            jsonStr = json!({
                "branch": "master",
                "status": "success",
                "message": message
            }).to_string();
        },
        _ => {
            jsonStr = json!({
                "branch": "",
                "status": "error",
                "message": message
            }).to_string();
        }
    }

    dbg!(jsonStr);    
    // dbg!(curr_branch.shorthand().unwrap());

    // let mut options = &mut StatusOptions::new();
    // options = options.include_untracked(true);
    // let statusArr = match repo.statuses(Some(options)) {
    //     Ok(statusArr) => statusArr,
    //     Err(e) => panic!("failed to get status: {}", e),
    // };

    // let statusIter = statusArr.iter();

    // let mut responseVector: Vec<HashMap<String, Vec<u32>>> = Vec::new();

    // for elem in statusIter {

    //     let status_val = elem.status().bits();
    //     let mut i = 1;
    //     let mut iter = 1;
    //     let mut flag_arr: Vec<u32> = vec![];
    //     if status_val == 0 {
    //         flag_arr.push(0)
    //     } else{
    //         while i<=32768 {
    //             if status_val & i > 0 {
    //                flag_arr.push(iter);
    //             }
    //             i = i << 1;
    //             iter = iter + 1;
    //         }
    //     }
    //     let mut file_obj = HashMap::new();
    //     file_obj.insert(elem.path().unwrap().into(), flag_arr);
    //     println!("Status: {:?}, Flename: {:?}", status_val, elem.path().unwrap());
    //     responseVector.push(file_obj);
    // }
    // let json = json!(responseVector).to_string();
    // dbg!(json);

    return;
}

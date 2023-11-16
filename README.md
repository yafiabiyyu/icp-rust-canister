
// Yafi Cansiter call

// add new education
dfx --identity Yafi canister call icp_rust_boilerplate_backend add_education_history '(record {name="Politeknik Negeri Malang";degre="D4";field_of_study="Teknik Informatika";start_year=2017;end_year=2021;})'
// get education list
dfx --identity Yafi canister call icp_rust_boilerplate_backend get_education_history
// update education data
dfx --identity Yafi canister call icp_rust_boilerplate_backend update_education_history '(0,record {name="Politeknik Negeri Malang";degre="D4";field_of_study="Teknik Informatika";start_year=2017;end_year=2022;})'
// remove education data
dfx --identity Yafi canister call icp_rust_boilerplate_backend delete_education_history '(0)'
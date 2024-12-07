type operation_line = { total : int; equation : int list }

let rec string_list_to_int_list (input : string list) (output : int list) :
    int list =
  match input with
  | some :: rest ->
      let some_trimmed = String.trim some in
      if some_trimmed <> "" then
        string_list_to_int_list rest (int_of_string some_trimmed :: output)
      else string_list_to_int_list rest output
  | [] -> List.rev output

let safe_int_of_string s =
  try Some (int_of_string (String.trim s)) with Failure _ -> None

let read_lines (file_name : string) : string list =
  In_channel.with_open_text file_name In_channel.input_lines

let rec read_list (data : string list) (res : operation_line list) :
    operation_line list =
  match data with
  | elem :: rest -> (
      let vals = String.split_on_char ':' elem in
      match vals with
      | front :: bot :: _ -> (
          let trimmed = String.trim front in
          match safe_int_of_string trimmed with
          | Some front_parsed ->
              let bot_vals = String.split_on_char ' ' bot in
              let bot_parsed = string_list_to_int_list bot_vals [] in
              read_list rest
                ({ total = front_parsed; equation = bot_parsed } :: res)
          | None -> read_list rest res)
      | _ -> read_list rest res)
  | [] -> List.rev res

let concat_ints (a : int) (b : int) : int =
  let a_str = string_of_int a in
  let b_str = string_of_int b in
  let str_list = [ a_str; b_str ] in
  let concat = String.concat "" str_list in
  int_of_string concat

let rec total_can_be_reached (total : int) (vals : int list) (curr : int) : bool
    =
  match vals with
  | first :: rest ->
      total_can_be_reached total rest (curr + first)
      || total_can_be_reached total rest (curr * first)
      || total_can_be_reached total rest (concat_ints curr first)
  | _ -> total = curr

let operation_is_valid (test : operation_line) : bool =
  total_can_be_reached test.total test.equation 0

let get_operation_total (op : operation_line) : int = op.total

let () =
  let lines = read_lines "data/input" in
  let operations = read_list lines [] in
  let sum =
    List.filter operation_is_valid operations
    |> List.map get_operation_total
    |> List.fold_left ( + ) 0
  in
  let sum_str = string_of_int sum in
  print_endline sum_str


open Base
open Solution

module Solution : Solve = struct
  type t = (char list -> char list) -> string

  let parse input =
    let hd, tl =
      Stdio.In_channel.input_lines input
      |> List.split_while ~f:(Fn.non String.is_empty)
    in
    let stacks =
      let stack_of i =
        let extract_crate s =
          String.get s ((i * 4) + 1) |> function
          | 'A' .. 'Z' as c -> Some c
          | _ -> None
        in
        List.filter_map hd ~f:extract_crate
      in
      let len = ((List.last_exn hd |> String.length) + 1) / 4 in
      List.range 0 len |> List.map ~f:stack_of
    in
    let procedure =
      let parse_procedure s =
        Caml.Scanf.sscanf s "move %d from %d to %d" (fun num from_idx to_idx ->
            (num, from_idx - 1, to_idx - 1))
      in
      List.drop tl 1 |> List.map ~f:parse_procedure
    in
    fun f ->
      let move stacks (num, src, dst) =
        let hd, tl = List.split_n stacks.(src) num in
        stacks.(dst) <- f hd @ stacks.(dst);
        stacks.(src) <- tl;
        stacks
      in
      List.fold procedure ~init:(Array.of_list stacks) ~f:move
      |> Array.filter_map ~f:List.hd
      |> Array.to_list |> String.of_char_list

  let part1 top_crates = top_crates List.rev |> answer_of_string
  let part2 top_crates = top_crates Fn.id |> answer_of_string
end

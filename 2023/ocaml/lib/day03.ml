open Base
open Core
open Solution

module Solution : Solve = struct
  module P = Tuple.Hashable (Int) (Int)

  type t = int list list

  let parse input =
    let part_numbers = P.Table.create () in
    let schematic = In_channel.input_lines input in
    List.iteri schematic ~f:(fun i line ->
        String.iteri line ~f:(fun j c ->
            if not Char.(c = '.' || is_digit c) then
              Hashtbl.add_exn part_numbers ~key:(i, j) ~data:[]));
    List.concat_mapi schematic ~f:(fun i line ->
        let re = Str.regexp {|[0-9]+|} in
        let rec f start =
          match Str.search_forward re line start with
          | j ->
              let s = Str.matched_string line in
              ((i, j), s) :: f (j + String.length s)
          | exception Stdlib.Not_found -> []
        in
        f 0)
    |> List.iter ~f:(fun ((i, j), s) ->
           let f (i, j) =
             Hashtbl.change part_numbers (i, j)
               ~f:(Option.map ~f:(List.cons @@ Int.of_string s))
           in
           List.cartesian_product
             (List.range (i - 1) (i + 2))
             (List.range (j - 1) (j + 1 + String.length s))
           |> List.iter ~f);
    Hashtbl.data part_numbers

  let part1 part_numbers =
    List.sum (module Int) part_numbers ~f:(List.reduce_exn ~f:( + ))
    |> Answer.of_int

  let part2 part_numbers =
    List.filter part_numbers ~f:(fun numbers -> List.length numbers = 2)
    |> List.sum (module Int) ~f:(List.reduce_exn ~f:( * ))
    |> Answer.of_int
end

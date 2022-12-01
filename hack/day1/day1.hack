use namespace HH\Lib\{C, File, Str, Vec};

<<__EntryPoint>>
async function main(): Awaitable<void> {
    $input_file = await File\open_read_only('input.txt')->readAllAsync();
    $elves_calories = $input_file
        |> Str\split($$, "\n\n")
        |> Vec\map(
            $$,
            $inventory ==> $inventory
                |> Str\split($$, "\n")
                |> C\reduce($$, ($total_calories, $calories) ==> $total_calories + (int)$calories, 0),
        )
        |> Vec\sort($$, ($a, $b) ==> $b <=> $a);

    echo Str\format("The highest total calories is %d\n", $elves_calories[0]);
    echo Str\format("The top three total calories is: %d\n", $elves_calories[0] + $elves_calories[1] + $elves_calories[2]);
}

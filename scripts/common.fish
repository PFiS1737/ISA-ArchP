function echo_h1
    set_color blue
    echo $argv[1]
    echo (string replace -ar "." "=" $argv[1])
    set_color normal
end

function echo_h2
    set_color green
    echo $argv[1]
    set_color normal
end

function echo_warning
    set_color yellow
    echo $argv[1]
    set_color normal
end

function echo_error
    set_color red
    echo $argv[1]
    set_color normal
end

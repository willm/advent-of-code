<?php declare(strict_types=1);

final class Window {
    private $window;
    private $height_map;
    private $x;
    private $y;

    function __construct(&$height_map, $x, $y) {
        $this->height_map = &$height_map;
        $window_x = range($x -1, $x + 1);
        $window_y = range($y -1, $y + 1);
        $this->window = array_map(function ($x) use($window_y, $height_map) {
            return array_map(function ($y) use ($x, $height_map): ?int {
                if ($x < 0 || $y < 0 || $x > count($height_map) - 1 || $y > count($height_map[$x]) - 1) {
                    return 9;
                }
                return $height_map[$x][$y];
            }, $window_y);
        }, $window_x);
        $this->x = $x;
        $this->y = $y;
    }

    public function get(): array {
        return $this->window;
    }

    public function point(): array {
        return array($this->x, $this->y);
    }

    public function is_next_to(Window $window): bool {
        return ($this->x == $window->x - 1 && $this->y == $window->y) ||
            ($this->x == $window->x + 1 && $this->y == $window->y) ||
            ($this->x == $window->x && $this->y == $window->y - 1) ||
            ($this->x == $window->x && $this->y == $window->y + 1);
    }

    public function is_low_point(): bool
    {
        $mid = $this->window[1][1];
        $west = $this->window[1][2];
        $east = $this->window[1][0];
        $north = $this->window[0][1];
        $south = $this->window[2][1];
        $real_points = array($west, $east, $north, $south);
        return count($real_points) > 0 && $mid < min($real_points);
    }

    public function __toString() {
        return "$this->x,$this->y";
    }
}

final class HeightMap {
    public static function from_file(string $path): array {
        $lines = file($path);
        return array_map(function ($line): array {
            $line = str_replace("\n", "", $line);
            return array_map(function($char): int {
                    return (int) $char;
                },
                str_split($line)
            );
        }, $lines);
    }
}

final class RiskLevel {
    public static function get(array $low_points, array &$height_map): int {
        return array_reduce($low_points, function($acc, $point) use ($height_map) {
            $height = $height_map[$point[0]][$point[1]];
            return $acc + (1 + $height);
        }, 0);
    }
}

function containing_index(array $basins, Window $window): ?int {
    for ($i = 0; $i < count($basins); $i++) {
        for ($j = 0; $j < count($basins[$i]); $j++) {
            if ($basins[$i][$j]->is_next_to($window)) {
                return $i;
            }
        }
    }
    return null;
}

final class Basin {
    private $windows;
    function __construct($windows) {
        $this->windows = $windows;
    }

    public function get_windows() {
        return $this->windows;
    }

    public function is_connecting(Basin $other): bool {
        foreach($this->get_windows() as $a) {
            foreach($other->get_windows() as $b) {
                if ($a->is_next_to($b)) {
                    return true;
                }
            }
        }
        return false;
    }

    public static function merge($basins) {
        $points = array();
        foreach($basins as $basin) {
            foreach($basin->get_windows() as $window) {
                if (!in_array($window, $points)) {
                    array_push($points, $window);
                }
            }
        }
        return new Basin($points);
    }

    public function __toString() {
        sort($this->windows);
        $str = "";
        foreach($this->windows as $w){
            $str = "$str|$w";
        }
        return $str;
    }
}

final class SmokeBasin
{
    public static function get_total_risk_level(string $path): int {
        $height_map = HeightMap::from_file($path);
        $low_points = SmokeBasin::get_low_points($height_map);
        return RiskLevel::get($low_points, $height_map);
    }

    public static function get_low_points_from_file(string $path): array
    {
        $height_map = HeightMap::from_file($path);
        return SmokeBasin::get_low_points($height_map);
    }

    public static function get_low_points(array &$height_map): array {
        $points = array();
        for ($x = 0; $x < count($height_map); $x++) {
            for ($y = 0; $y < count($height_map[$x]); $y++) {
                $window = new Window($height_map, $x, $y);
                if ($window->is_low_point($window)) {
                    array_push($points, array($x, $y));
                }
            }
        }
        return $points;
    }

    public static function get_basins(&$height_map): array {
        $basin_windows = array();
        for ($x = 0; $x < count($height_map); $x++) {
            for ($y = 0; $y < count($height_map[$x]); $y++) {
                $window = new Window($height_map, $x, $y);
                if ($height_map[$x][$y] == 9) {
                    continue;
                }
                $basin = containing_index($basin_windows, $window);
                if (!is_null($basin)) {
                    //echo "\n adding to existing basin";
                    array_push($basin_windows[$basin], $window);
                    //echo "\n new size: ".count($basins[$basin]);
                } else {
                    //echo "\n adding basin";
                    array_push($basin_windows, array($window));
                }
            }
        }
        $basins = array_map(function ($basin) {
            return new Basin($basin);
        }, $basin_windows);

        return merge($basins, $height_map);
    }

}

function multiple_of_3_largest($basins) {
    function by_window_count($a, $b) {
        if (count($a->get_windows()) == count($b->get_windows())) {
            return 0;
        }
        return (count($a->get_windows()) < count($b->get_windows())) ? 1 : -1;
    }
    usort($basins, 'by_window_count');
    return array_reduce(array_slice($basins, 0, 3), function ($size, $basin) {
        $c = count($basin->get_windows());
        if (is_null($size)) {
            return count($basin->get_windows());
        }
        return $size * count($basin->get_windows());
    }, null);
}

function merge($basins, &$height_map) {
    $merged = [];
    for ($i = 0; $i < count($basins); $i++) {
        for ($j = 0; $j < count($basins); $j++) {
            if ($i != $j && $basins[$i]->is_connecting($basins[$j])) {
                array_push($merged, Basin::merge([$basins[$i], $basins[$j]]));
                for($k = 0; $k < count($basins); $k++) {
                    if ($k != $i && $k != $j) {
                        array_push($merged, $basins[$k]);
                    }
                }
                return merge($merged, $height_map);
            }
        }
    }
    return $basins;
}


if (count($argv) > 1) {
    $height_map = HeightMap::from_file($argv[1]);
    $basins = SmokeBasin::get_basins($height_map);
    echo(multiple_of_3_largest($basins));
}

function binarySearch(array, val)
    local low = 0
    local high = #array-1

    while low <= high do
        local mid = math.floor(low + (high - low) / 2)
        local current = array[mid]
        
        if current == val then
            return mid
        end

        if current <= val then
            low = mid + 1
        else
            high = mid - 1
        end
    end

    return -1
end

function setup(n)
    local array = {}

    for i=0,n do
        array[i] = i
    end

    return array
end

function bench(_n, array)
    -- the value -1 never appears, should always run in worst possible time.
    binarySearch(array, -1)
end
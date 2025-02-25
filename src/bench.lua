function benchTime(callback, n)
    local start = os.clock()
    callback(n)
    return os.clock()-start
end

function benchAvgTime(callback, n, repeats)
    local sum = 0

    for i=0,repeats do
        sum += benchTime(callback, n)
    end
    
    return sum / repeats
end
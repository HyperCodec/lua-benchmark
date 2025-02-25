function benchTime(bench, setup, n)
    if setup ~= nil then
        local data = setup(n)

        local start = os.clock()
        bench(n, data)
        return os.clock()-start
    end

    local start = os.clock()
    bench(n)
    return os.clock()-start
end

function benchAvgTime(bench, setup, n, repeats)
    local sum = 0

    for i=0,repeats do
        sum += benchTime(bench, setup, n)
    end
    
    return sum / repeats
end
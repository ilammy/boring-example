import SwiftUI

func sha256(_ input: Data) -> Data {
    let inputBytes = UnsafeMutableBufferPointer<UInt8>.allocate(capacity: input.count)
    let inputCount = input.copyBytes(to: inputBytes)
    let outputBytes = UnsafeMutableBufferPointer<UInt8>.allocate(capacity: 32)
    var outputCount = outputBytes.count
    if !compute_sha_256(inputBytes.baseAddress, inputCount, outputBytes.baseAddress, &outputCount) {
        fatalError("failed to compute SHA-256")
    }
    if outputCount != outputBytes.count {
        fatalError("output is not 32 bytes")
    }
    return Data(outputBytes)
}

func sha256_hex(_ s: String) -> String {
    let input = Data(s.utf8)
    let output = sha256(input)
    return output.map { String(format: "%02x", $0) }.joined()
}

struct ContentView: View {
    var body: some View {
        let string = "abc"
        let sha256 = sha256_hex(string)
        Text("SHA-256 of \"\(string)\" is \(sha256)")
            .padding()
    }
}

struct ContentView_Previews: PreviewProvider {
    static var previews: some View {
        ContentView()
    }
}

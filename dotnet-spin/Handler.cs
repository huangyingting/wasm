using Fermyon.Spin.Sdk;
using System.Text;
using System.Net;
using System.Runtime.InteropServices;

namespace DotnetSpin;

public static class Handler
{
    [HttpHandler]
    public static HttpResponse HandleHttpRequest(HttpRequest request)
    {
        var responseText = new StringBuilder();
        responseText.AppendLine($":method: {request.Method}");

        foreach (var h in request.Headers)
        {
            responseText.AppendLine($"{h.Key}: {h.Value}");
        }

        var bodyInfo = request.Body.HasContent() ?
            $"body: {request.Body.AsString()}\n" :
            "\n";
        responseText.AppendLine(bodyInfo);
        responseText.AppendLine($"arch: {RuntimeInformation.OSArchitecture}");
        responseText.AppendLine($"framework: {RuntimeInformation.FrameworkDescription}");
        responseText.AppendLine($"version: {Environment.GetEnvironmentVariable("VERSION")}");


        return new HttpResponse
        {
            StatusCode = HttpStatusCode.OK,
            Headers = new Dictionary<string, string>
            {
                { "Content-Type", "text/plain" },
            },
            BodyAsString = responseText.ToString(),
        };    
    }
}

<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Update_user</name>
   <tag></tag>
   <elementGuidId>6dadeed9-aa52-41b1-9e36-c1e0f5d2889d</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;active\&quot;: true,\n  \&quot;address\&quot;: \&quot;string\&quot;,\n  \&quot;city\&quot;: \&quot;string\&quot;,\n  \&quot;country\&quot;: \&quot;string\&quot;,\n  \&quot;creationDate\&quot;: \&quot;2022-02-15T20:23:41.149Z\&quot;,\n  \&quot;creator\&quot;: 0,\n  \&quot;dailyEmail\&quot;: \&quot;string\&quot;,\n  \&quot;documentNumber\&quot;: \&quot;string\&quot;,\n  \&quot;documentType\&quot;: {\n    \&quot;abbreviation\&quot;: \&quot;string\&quot;,\n    \&quot;id\&quot;: 0,\n    \&quot;name\&quot;: \&quot;string\&quot;\n  },\n  \&quot;documentationStatus\&quot;: true,\n  \&quot;documentationSupport\&quot;: \&quot;string\&quot;,\n  \&quot;email\&quot;: \&quot;string\&quot;,\n  \&quot;firstLastName\&quot;: \&quot;string\&quot;,\n  \&quot;firstName\&quot;: \&quot;string\&quot;,\n  \&quot;id\&quot;: 0,\n  \&quot;indicative\&quot;: \&quot;string\&quot;,\n  \&quot;lastModifiedBy\&quot;: 0,\n  \&quot;mobile\&quot;: \&quot;string\&quot;,\n  \&quot;phone\&quot;: \&quot;string\&quot;,\n  \&quot;secondLastName\&quot;: \&quot;string\&quot;,\n  \&quot;secondName\&quot;: \&quot;string\&quot;,\n  \&quot;state\&quot;: \&quot;string\&quot;,\n  \&quot;userType\&quot;: {\n    \&quot;id\&quot;: 0,\n    \&quot;name\&quot;: \&quot;string\&quot;\n  }\n}&quot;,
  &quot;contentType&quot;: &quot;text/plain&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Accept</name>
      <type>Main</type>
      <value>*/*</value>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Connection</name>
      <type>Main</type>
      <value>keep-alive</value>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Accept-Encoding</name>
      <type>Main</type>
      <value>gzip, deflate, br</value>
   </httpHeaderProperties>
   <katalonVersion>8.2.5</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>PUT</restRequestMethod>
   <restUrl>https://unit-test.midaily.co/api/v1/user/update</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>

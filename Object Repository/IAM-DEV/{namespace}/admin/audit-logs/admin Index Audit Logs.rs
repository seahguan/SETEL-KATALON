<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>admin Index Audit Logs</name>
   <tag></tag>
   <elementGuidId>8920706a-1f17-46b5-8013-40875349623f</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>true</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>access-token</name>
      <type>Main</type>
      <value></value>
      <webElementGuid>5b3c334f-9018-4999-81a0-53a0f1752676</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Accept</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>0aef9ddd-a1c3-4372-a4dc-5dffa13153a2</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>9.0.0</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>${baseUrl}/:namespace/admin/audit-logs?types=SessionPool&amp;types=Group&amp;actions=Create&amp;actions=Create&amp;identifier=&lt;string>&amp;createdBy=&lt;string>&amp;sourceNamespaces=&lt;string>&amp;sourceNamespaces=&lt;string>&amp;createdAtFrom=&lt;dateTime>&amp;createdAtTo=&lt;dateTime>&amp;perPage=&lt;number>&amp;page=&lt;number></restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.baseUrl</defaultValue>
      <description></description>
      <id>b9908915-2ad5-4215-86e0-22085fd5e696</id>
      <masked>false</masked>
      <name>baseUrl</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
#[doc = "Register `SCMER` reader"]
pub type R = crate::R<ScmerSpec>;
#[doc = "Register `SCMER` writer"]
pub type W = crate::W<ScmerSpec>;
#[doc = "Field `SCME` reader - desc SCME"]
pub type ScmeR = crate::FieldReader<u16>;
#[doc = "Field `SCME` writer - desc SCME"]
pub type ScmeW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - desc SCME"]
    #[inline(always)]
    pub fn scme(&self) -> ScmeR {
        ScmeR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - desc SCME"]
    #[inline(always)]
    pub fn scme(&mut self) -> ScmeW<'_, ScmerSpec> {
        ScmeW::new(self, 0)
    }
}
#[doc = "desc SCMER\n\nYou can [`read`](crate::Reg::read) this register and get [`scmer::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scmer::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ScmerSpec;
impl crate::RegisterSpec for ScmerSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scmer::R`](R) reader structure"]
impl crate::Readable for ScmerSpec {}
#[doc = "`write(|w| ..)` method takes [`scmer::W`](W) writer structure"]
impl crate::Writable for ScmerSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCMER to value 0xffff"]
impl crate::Resettable for ScmerSpec {
    const RESET_VALUE: u32 = 0xffff;
}

#[doc = "Register `PR` reader"]
pub type R = crate::R<PrSpec>;
#[doc = "Register `PR` writer"]
pub type W = crate::W<PrSpec>;
#[doc = "Field `I2SDIV` reader - desc I2SDIV"]
pub type I2sdivR = crate::FieldReader;
#[doc = "Field `I2SDIV` writer - desc I2SDIV"]
pub type I2sdivW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - desc I2SDIV"]
    #[inline(always)]
    pub fn i2sdiv(&self) -> I2sdivR {
        I2sdivR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - desc I2SDIV"]
    #[inline(always)]
    pub fn i2sdiv(&mut self) -> I2sdivW<'_, PrSpec> {
        I2sdivW::new(self, 0)
    }
}
#[doc = "desc PR\n\nYou can [`read`](crate::Reg::read) this register and get [`pr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PrSpec;
impl crate::RegisterSpec for PrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr::R`](R) reader structure"]
impl crate::Readable for PrSpec {}
#[doc = "`write(|w| ..)` method takes [`pr::W`](W) writer structure"]
impl crate::Writable for PrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PR to value 0x02"]
impl crate::Resettable for PrSpec {
    const RESET_VALUE: u32 = 0x02;
}
